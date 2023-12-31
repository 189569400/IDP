# This sample checks that isinstance and issubclass don't
# allow the second argument to be a Protocol class.

from typing import Any, Callable, Literal, Protocol, Type, TypeVar, Union
from types import FunctionType, LambdaType


class MyProtocol(Protocol):
    pass


# This should generate an error because Sized is a Protocol that
# is not runtime checkable.
isinstance(4, MyProtocol)


# This should generate an error because Iterable is a Protocol.
issubclass(str, (str, MyProtocol))


class CustomClass:
    def __call__(self, *args: Any):
        pass


def get_type_of_object(object: Union[Callable[..., Any], CustomClass]):
    # This would normally generate an error, but FunctionType is special.
    if isinstance(object, FunctionType):
        return "is function"

    if isinstance(object, LambdaType):
        return "is lambda"

    if isinstance(object, Callable):
        return "is callable"

    return "nothing"


_T = TypeVar("_T", bound=CustomClass)


def func(cls: Type[_T], val: _T):
    if issubclass(cls, CustomClass):
        t1: Literal["Type[CustomClass]*"] = reveal_type(cls)
    else:
        t2: Literal["Never"] = reveal_type(cls)
