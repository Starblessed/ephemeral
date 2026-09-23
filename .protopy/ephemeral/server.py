import asyncio
import socket
from datetime import UTC, datetime
from socket import AF_INET, SO_REUSEADDR, SOCK_STREAM, SOL_SOCKET

from ephemeral.session_manager import SessionManager


class Server:
    def __init__(self, ip: str, port: int):
        self._started: datetime = datetime.now(tz=UTC)

        self.manager: SessionManager = SessionManager()

        self.ip: str = ip
        self.port: int = port

    async def run(self) -> None:
        with socket.socket(AF_INET, SOCK_STREAM) as server:
            server.setsockopt(SOL_SOCKET, SO_REUSEADDR, 1)
            server.bind((self.ip, self.port))
            server.listen()
            server.setblocking(False)

            loop: asyncio.AbstractEventLoop = asyncio.get_event_loop()
            print(
                f"Listening on {self.ip}:{self.port}"
            )  # TODO: replace print by logger.INFO

            while True:
                client_socket, address = await loop.sock_accept(server)
                print(f"Connected: {address}")  # TODO: replace print by logger.INFO

                self.manager.add_socket(client_socket)
