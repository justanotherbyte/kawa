from typing import (
    Optional,
    Dict,
    List
)


class Url:
    host: str
    port: int
    scheme: str
    path: str

    def __new__(self, url: str) -> Url: ...

class Request:
    def __new__(self, method: str, url: Url) -> Request: ...
    def add_header(self, header: str, value: str): ...
    def set_body(self, body: str): ...
    def create_message(self) -> List[int]: ...
    def send(self) -> Response: ...

class Response:
    status: int
    body: str
    headers: Dict[str, str]

    def __repr__(self) -> str: ...


def get(url: str, headers: Dict[str, str]): ...
def post(url: str, headers: Dict[str, str], body: Optional[str] = None): ...
def put(url: str, headers: Dict[str, str], body: Optional[str] = None): ...
def delete(url: str, headers: Dict[str, str], body: Optional[str] = None): ...