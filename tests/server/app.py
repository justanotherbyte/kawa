from typing import Any

from litestar import (
    Litestar,
    get,
    post
)


@get("/")
async def hello() -> str:
    return "hello world"

@post("/data")
async def echo_data(data: dict) -> dict:
    return data

@post("/create")
async def create() -> str:
    return "created"

app = Litestar([create, echo_data, hello])