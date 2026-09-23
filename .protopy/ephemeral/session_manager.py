import asyncio

from ephemeral.session import Session


class SessionManager:
    def __init__(self):
        self.reader: asyncio.StreamReader
        self.writer: asyncio.StreamWriter
        self.session: Session
