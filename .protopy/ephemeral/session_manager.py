import asyncio
import json
import socket
from typing import Any

from ephemeral.session import Session


class SessionManager:
    def __init__(self):
        self.sessions: dict[str, Session] = {}
        self.tasks: set[asyncio.Task[None]] = set()

    def add_socket(self, sock: socket.socket) -> Session:
        sock.setblocking(False)

        session = Session(id_length=16)
        session.socket = sock
        session.set_data({})

        self.sessions[session.id] = session

        task = asyncio.create_task(self.run_session(session=session))

        self.tasks.add(task)

        task.add_done_callback(self.tasks.discard)

        return session

    async def run_session(self, session: Session):
        sock = session.socket
        if sock is None:
            raise ValueError(f"No socket found for session {session.id}")

        buffer: bytearray = bytearray()

        loop: asyncio.AbstractEventLoop = asyncio.get_running_loop()

        try:
            while True:
                chunk = await loop.sock_recv(sock, 4096)

                if not chunk:
                    break

                buffer.extend(chunk)

                while b"\n" in buffer:
                    line, _, remaining = buffer.partition(b"\n")
                    buffer = bytearray(remaining)

                    request = self.parse_request(bytes(line))

                    response = await self.handle_request(session, request)

                    await self.send(session, response)

        finally:
            sock.close()
            session.socket = None
            self.sessions.pop(session.id, None)

    def parse_request(self, raw: bytes) -> dict[str, Any]:
        request = json.loads(raw)

        if not isinstance(request, dict):
            raise TypeError("Expected a JSON object")

        return request

    async def handle_request(
        self, session: Session, request: dict[str, Any]
    ) -> dict[str, Any]:

        command: str | None = request.get("cmd", None)
        payload: str | None = request.get("payload", None)

        if command == None or payload == None:
            raise ValueError("Neither cmd nor payload can be None!")

        match command:
            case "get":
                # No payload needed
                try:
                    return {"result": session.get_data()}

                except ValueError as exc:
                    return {"error": str(exc)}

            case "get_partial":
                # payload must be a list of keys
                if not isinstance(payload, list):
                    return {"error": "get requires a list of keys"}

                try:
                    return {"result": session.get_partial_data(keys=payload)}

                except (ValueError, KeyError) as exc:
                    return {"error": str(exc)}

            case "set":
                # payload must be a dict
                if not isinstance(payload, dict):
                    return {"error": "set requires an object"}

                return {"result": session.set_data(data=payload)}

            case "set_partial":
                # payload must be a dict
                if not isinstance(payload, dict):
                    return {"error": "set_partial requires an object"}

                try:
                    return {"result": session.set_partial_data(data=payload)}

                except ValueError as exc:
                    return {"error": str(exc)}

            case _:
                return {"error": f'invalid command: "{command!r}"'}

    async def send(self, session: Session, response: dict[str, Any]) -> None:
        sock = session.socket

        if sock is None:
            raise ConnectionError("Session is closed")

        payload = json.dumps(response).encode() + b"\n"

        await asyncio.get_running_loop().sock_sendall(sock, payload)
