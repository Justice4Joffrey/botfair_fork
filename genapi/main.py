#!/usr/bin/env python3

from typing import Optional, List, Tuple
from xml.etree.ElementTree import Element, parse
from enum import Enum, auto
from dataclasses import dataclass
from dataclasses_json import DataClassJsonMixin

import string


@dataclass
class Value(DataClassJsonMixin):
    """A value as per BF API"""

    name: str
    _id: Optional[int]
    description: Optional[str]


@dataclass
class Param(DataClassJsonMixin):
    """A parameter as per the BF API"""

    name: str
    _type: str
    description: Optional[str]
    mandatory: bool
    values: Optional[List[Value]]


@dataclass
class SimpleResponse(DataClassJsonMixin):
    """A simple response"""

    _type: str
    description: Optional[str]


@dataclass
class ExceptionResponse(DataClassJsonMixin):
    """A simple exception response"""

    _type: str
    description: Optional[str]


@dataclass
class Operation(DataClassJsonMixin):
    """An operation as per the BF API"""

    name: str
    since: str
    description: Optional[str]
    params: List[Param]
    simple_response: SimpleResponse
    exceptions: List[ExceptionResponse]


@dataclass
class ExceptionType(DataClassJsonMixin):
    """An exception type as per the BF API"""

    name: str
    prefix: str
    params: List[Param]


@dataclass
class SimpleType(DataClassJsonMixin):
    """An exception type as per the BF API"""

    name: str
    _type: str
    description: Optional[str]
    values: Optional[List[Value]]


@dataclass
class DataType(DataClassJsonMixin):
    """An exception type as per the BF API"""

    name: str
    description: Optional[str]
    params: List[Param]


@dataclass
class APING(DataClassJsonMixin):
    """Parsed Betfair APING"""

    description: Optional[str]
    operations: List[Operation]
    data_types: List[DataType]
    exception_types: List[ExceptionType]
    simple_types: List[SimpleType]


class ParentType(Enum):
    OPERATION = auto()
    DATA_TYPE = auto()
    EXCEPTION_TYPE = auto()
    SIMPLE_TYPE = auto()


def parse_type(_type: str) -> str:
    _type = _type.replace(" ", "")  # sanity

    allowable: str = string.ascii_letters + string.digits

    if _type.startswith("list("):
        inner_type = _type.replace(")", "", 1).split("(", maxsplit=1)[1]
        assert all(x in allowable for x in inner_type)
        _type = f"List[{inner_type}]"

    elif _type.startswith("set("):
        inner_type = _type.replace(")", "", 1).split("(", maxsplit=1)[1]
        assert all(x in allowable for x in inner_type)
        _type = f"Set[{inner_type}]"

    elif _type.startswith("map("):
        inner_type_1, inner_type_2 = (
            _type.replace(")", "", 1)
            .split("(", maxsplit=1)[1]
            .split(",", maxsplit=1)
        )
        assert all(x in allowable for x in inner_type_1), inner_type_1
        assert all(x in allowable for x in inner_type_2), inner_type_2
        _type = f"Map[{inner_type_1}, {inner_type_2}]"

    return _type


def strip_string(s: Optional[str]) -> Optional[str]:
    if s is None:
        return None

    return " ".join([x.strip() for x in s.split("\n")]).strip()


def parse_description(el: Element) -> Optional[str]:
    assert el.tag == "description"
    assert not el.attrib
    assert len(el) == 0

    if el.text is None:
        return None

    return strip_string(el.text)


def parse_exceptions(el: Element) -> List[ExceptionResponse]:
    assert el.tag == "exceptions"
    assert not el.attrib
    assert not strip_string(el.text)

    exceptions: List[ExceptionResponse] = []
    for child in el:  # type: Element
        if child.tag == "exception":
            _type = child.attrib.pop("type")
            assert not child.attrib
            assert not strip_string(child.text)
            assert len(child) == 1

            _type = parse_type(_type)
            description = parse_description(child[0])
            exceptions.append(
                ExceptionResponse(_type=_type, description=description)
            )
            continue

        raise NotImplementedError(child.tag)

    return exceptions


def parse_value(el: Element, parent: ParentType) -> Value:
    name = el.attrib.pop("name")

    _id: Optional[int]
    if parent == ParentType.EXCEPTION_TYPE:
        _id_str: str = el.attrib.pop("id")
        assert _id_str.isdigit()
        _id = int(_id_str)

    elif parent == ParentType.SIMPLE_TYPE:
        _id = None

    else:
        raise NotImplementedError(f"value: {parent}")

    assert not el.attrib, el.attrib
    assert not strip_string(el.text)

    assert len(el) == 1
    description = parse_description(el[0])

    return Value(name=name, _id=_id, description=description)


def parse_validValues(el: Element, parent: ParentType) -> List[Value]:
    assert not el.attrib
    assert not strip_string(el.text)

    values: List[Value] = []
    for child in el:  # type: Element
        if child.tag == "value":
            values.append(parse_value(child, parent=parent))
            continue

        raise NotImplementedError(child.tag)

    return values


def parse_parameter(el: Element, parent: ParentType) -> Param:
    mandatory: bool
    try:
        mandatory_str = el.attrib.pop("mandatory")
        assert mandatory_str == "true"
        mandatory = True
    except Exception:
        mandatory = False

    name = el.attrib.pop("name")
    _type = el.attrib.pop("type")
    _type = parse_type(_type)

    assert not el.attrib
    assert not strip_string(el.text)

    description: Optional[str] = None
    values: Optional[List[Value]] = None
    for child in el:  # type: Element
        if child.tag == "description":
            assert description is None
            description = parse_description(child)
            continue
        if parent == ParentType.EXCEPTION_TYPE and child.tag == "validValues":
            values = parse_validValues(child, parent=ParentType.EXCEPTION_TYPE)
            continue

        raise NotImplementedError(child.tag)

    return Param(
        name=name,
        _type=_type,
        description=description,
        mandatory=mandatory,
        values=values,
    )


def parse_request(el: Element) -> List[Param]:
    assert el.tag == "request"
    assert not el.attrib
    assert not strip_string(el.text)

    params: List[Param] = []
    for child in el:  # type: Element
        if child.tag == "parameter":
            param = parse_parameter(child, parent=ParentType.OPERATION)
            params.append(param)
            continue

        raise NotImplementedError(child.tag)

    return params


def parse_parameters(
    el: Element
) -> Tuple[List[Param], SimpleResponse, List[ExceptionResponse]]:
    assert el.tag == "parameters"
    assert not el.attrib
    assert not strip_string(el.text)

    for child in el:  # type: Element
        if child.tag == "request":
            params = parse_request(child)
            continue
        if child.tag == "simpleResponse":
            _type = child.attrib.pop("type")
            assert not child.attrib
            assert not strip_string(child.text)

            assert len(child) == 1
            description = parse_description(child[0])

            simple_response = SimpleResponse(
                _type=parse_type(_type), description=description
            )
            continue
        if child.tag == "exceptions":
            exceptions = parse_exceptions(child)
            continue

        raise NotImplementedError(child.tag)

    return (params, simple_response, exceptions)


def parse_operation(el: Element) -> Operation:
    assert el.tag == "operation"
    name = el.attrib.pop("name")
    since = el.attrib.pop("since")
    assert not el.attrib
    assert not strip_string(el.text)

    for child in el:  # type: Element
        if child.tag == "description":
            assert not child.attrib
            assert len(child) == 0
            description = strip_string(child.text)
            continue
        if child.tag == "parameters":
            params, simple_response, exceptions = parse_parameters(child)
            continue

        raise NotImplementedError(child.tag)

    return Operation(
        name=name,
        since=since,
        description=description,
        params=params,
        simple_response=simple_response,
        exceptions=exceptions,
    )


def parse_dataType(el: Element) -> DataType:
    assert el.tag == "dataType"
    name = el.attrib.pop("name")
    assert not el.attrib
    assert not strip_string(el.text)

    description: Optional[str] = None
    params: List[Param] = []
    for child in el:  # type: Element
        if child.tag == "description":
            assert description is None
            description = parse_description(child)
            continue
        if child.tag == "parameter":
            params.append(parse_parameter(child, parent=ParentType.DATA_TYPE))
            continue

        raise NotImplementedError(child.tag)

    return DataType(name=name, description=description, params=params)


def parse_exceptionType(el: Element) -> ExceptionType:
    assert el.tag == "exceptionType"
    name = el.attrib.pop("name")
    prefix = el.attrib.pop("prefix")
    assert not el.attrib
    assert not strip_string(el.text)

    description: Optional[str] = None
    params: List[Param] = []
    for child in el:  # type: Element
        if child.tag == "description":
            assert description is None
            description = parse_description(child)
            continue
        if child.tag == "parameter":
            params.append(
                parse_parameter(child, parent=ParentType.EXCEPTION_TYPE)
            )
            continue

        raise NotImplementedError(child.tag)

    return ExceptionType(name=name, prefix=prefix, params=params)


def parse_simpleType(el: Element) -> SimpleType:
    assert el.tag == "simpleType"
    name = el.attrib.pop("name")
    _type = el.attrib.pop("type")
    assert not el.attrib
    assert not strip_string(el.text)

    values: Optional[List[Value]] = None
    description: Optional[str] = None
    for child in el:  # type: Element
        if child.tag == "description":
            assert description is None
            description = parse_description(child)
            continue
        if child.tag == "validValues":
            assert values is None
            values = parse_validValues(child, parent=ParentType.SIMPLE_TYPE)
            continue

        raise NotImplementedError(child.tag)

    return SimpleType(
        name=name, _type=_type, values=values, description=description
    )


def parse_aping(el: Element) -> APING:
    # We ignore the attributes here deliberately.
    assert not strip_string(el.text)

    description: Optional[str] = None
    operations: List[Operation] = []
    data_types: List[DataType] = []
    exception_types: List[ExceptionType] = []
    simple_types: List[SimpleType] = []
    for child in el:  # type: Element
        if child.tag == "description":
            assert description is None
            description = parse_description(child)
            continue
        if child.tag == "operation":
            operations.append(parse_operation(child))
            continue
        if child.tag == "dataType":
            data_types.append(parse_dataType(child))
            continue
        if child.tag == "exceptionType":
            exception_types.append(parse_exceptionType(child))
            continue
        if child.tag == "simpleType":
            simple_types.append(parse_simpleType(child))
            continue

        raise NotImplementedError(child.tag)

    return APING(
        description=description,
        operations=operations,
        data_types=data_types,
        exception_types=exception_types,
        simple_types=simple_types,
    )


def main() -> None:
    tree = parse("SportsAPING.xml")
    aping: APING = parse_aping(tree.getroot())
    print(aping.to_json())


if __name__ == "__main__":
    main()
