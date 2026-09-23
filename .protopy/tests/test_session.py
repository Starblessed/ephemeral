import pytest
from ephemeral import session as s


def test_set_get_data_round_trip():
    session = s.Session(16)

    data: dict = {"abc": 123, "xyz": 456}

    session.set_data(data=data)

    assert session.get_data() == data


def test_set_partial_data():
    session = s.Session(16)

    data: dict = {"abc": 123, "xyz": 456}

    session.set_data(data=data)

    new_data: dict = {"foo": 700}
    expected_data: dict = {"abc": 123, "xyz": 456, "foo": 700}

    session.set_partial_data(data=new_data)

    assert session.get_data() == expected_data


def test_set_partial_data_not_initialized():
    session = s.Session(16)

    data: dict = {"abc": 123, "xyz": 456}

    with pytest.raises(ValueError):
        session.set_partial_data(data=data)


def test_get_data_not_initialized():
    session = s.Session(16)

    with pytest.raises(ValueError):
        session.get_data()


def test_get_partial_data():
    session = s.Session(16)

    data: dict = {"abc": 123, "xyz": 456, "foo": 700}

    session.set_data(data=data)
    keys: list[str] = ["xyz", "foo"]

    expected_data: dict = {"xyz": 456, "foo": 700}

    assert session.get_partial_data(keys=keys) == expected_data


def test_get_partial_data_not_initialized():
    session = s.Session(16)

    with pytest.raises(ValueError):
        keys: list[str] = ["abc", "xyz"]
        session.get_partial_data(keys=keys)


def test_get_partial_data_key_not_in_data():
    session = s.Session(16)

    data: dict = {"abc": 123, "xyz": 456}
    keys: list[str] = ["abc", "foo", "www"]

    session.set_data(data=data)

    with pytest.raises(KeyError):
        session.get_partial_data(keys=keys)


def test_wipeout():
    session = s.Session(16)

    data: dict = {"abc": 123, "xyz": 456}

    session.set_data(data=data)

    session.wipeout()

    with pytest.raises(ValueError):
        session.get_data()
