from typing import TypeVar, Generic, Union, Optional, Protocol, Tuple, List, Any, Self
from types import TracebackType
from enum import Flag, Enum, auto
from dataclasses import dataclass
from abc import abstractmethod
import weakref

from ..types import Result, Ok, Err, Some



@dataclass
class Number_Signed:
    value: int


@dataclass
class Number_Unsigned:
    value: int


@dataclass
class Number_Float:
    value: float


Number = Union[Number_Signed, Number_Unsigned, Number_Float]



@dataclass
class SimpleValue_Null:
    pass


@dataclass
class SimpleValue_Boolean:
    value: bool


@dataclass
class SimpleValue_Num:
    value: Number


@dataclass
class SimpleValue_Str:
    value: str


SimpleValue = Union[SimpleValue_Null, SimpleValue_Boolean, SimpleValue_Num, SimpleValue_Str]



@dataclass
class Value_Null:
    pass


@dataclass
class Value_Boolean:
    value: bool


@dataclass
class Value_Num:
    value: Number


@dataclass
class Value_Str:
    value: str


@dataclass
class Value_Arr:
    value: List[SimpleValue]


@dataclass
class Value_Obj:
    value: List[Tuple[str, SimpleValue]]


Value = Union[Value_Null, Value_Boolean, Value_Num, Value_Str, Value_Arr, Value_Obj]



