from litestar import (
    Litestar,
    get,
    post
)


@get("/")
async def hello() -> str:
    return "hello world"

@get("/data")
async def data() -> dict:
    return {
        "username": "justanotherbyte",
        "password": "http"
    }

@post("/create")
async def create() -> str:
    return "created"

app = Litestar([create, data, hello])