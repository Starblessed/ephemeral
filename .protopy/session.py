from secrets import token_hex
from socket import socket
from typing import Any


class Session:
    # TODO: use gc and bytearray for better data wipe security.
    def __init__(self, id_length: int):
        self.id: str = token_hex(id_length)
        self.socket: socket | None = None
        self.data: dict | None = None

    def __check_has_data(self):
        if self.data is None:
            raise ValueError("Session data not yet initialized.")

    def set_data(self, data: dict) -> None:
        self.data = data

    def set_partial_data(self, data: dict) -> None:
        self.data.update(data)

    def get_data(self) -> dict:
        self.__check_has_data()
        return self.data

    def get_partial_data(self, keys: Any | list[Any]) -> dict:
        self.__check_has_data()
        _keys = keys if isinstance(keys, list) else [keys]
        try:
            return {k: self.data[k] for k in _keys}
        except KeyError:
            # Identifies orphan keys in the request
            orphan_keys: set[Any] = set(_keys) - set(self.data.keys())
            raise KeyError(f'Keys "{orphan_keys}" not present in session data!')

    def wipeout(self):
        del self.data
