import argparse
import asyncio

from ephemeral.server import Server


def main():
    parser = argparse.ArgumentParser(
        description="Ephemeral Server initialization parser"
    )

    parser.add_argument("ip", type=str, help="Host ip address")
    parser.add_argument("port", type=int, help="Server application port")

    args = parser.parse_args()

    server: Server = Server(ip=args.ip, port=args.port)  # type: ignore

    asyncio.run(server.run())


if __name__ == "__main__":
    main()
