import json
from socket import AF_INET, SOCK_STREAM, socket

from ephemeral.logger import get_logger
from ephemeral.server import STANDARD_PORT

logger = get_logger(name=__name__)


class Client:
    def __init__(self, server_addr: tuple[str, int]):
        self.server_addr: tuple[str, int] = server_addr
        self.socket: socket | None = None

    def connect(self):
        try:
            self.socket = socket(AF_INET, SOCK_STREAM)
            self.socket.connect(self.server_addr)
            logger.info(f"Connected to server at {self.server_addr}")
        except ConnectionRefusedError as exc:
            logger.error(f"Error: Could not connect to server: {exc!r}")

    def send(self, command: str, payload: list | dict | None = None):
        if self.socket is None:
            raise ValueError("Client is not connected.")

        message: str = json.dumps({"cmd": command, "payload": payload}) + "\n"

        self.socket.sendall(message.encode("utf-8"))
        logger.info(f"Message sent: {message}")

    def receive(self):
        if self.socket is None:
            raise ValueError("Client is not connected.")

        received_data: str = self.socket.recv(1024).decode("utf-8")
        logger.info(f"Received data: {received_data}")

        json_data: dict = json.loads(received_data)

        return json_data

    def set(self, data: dict):
        self.send(command="set", payload=data)
        response: dict = self.receive()
        return response

    def set_partial(self, data: dict):
        self.send(command="set_partial", payload=data)
        response: dict = self.receive()
        return response

    def get(self):
        self.send(command="get")
        data: dict = self.receive()
        return data

    def get_partial(self, keys: list[str]):
        self.send(command="get_partial", payload=keys)
        data: dict = self.receive()
        return data

    def disconnnect(self):
        if self.socket is not None:
            self.socket.close()


if __name__ == "__main__":
    client: Client = Client(("127.0.0.1", STANDARD_PORT))

    client.connect()

    res = client.set(data={"foo": 123})

    print(res)

    res = client.get_partial(keys=["foo"])

    print(res)

    client.disconnnect()
