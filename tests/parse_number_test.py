from fimficdata.common.util import parse_number
import pytest


# TODO: Now that this is parameterized, it's obvious that these are mostly
#   all the same test, just with different data. Can these be compressed
#   somehow, in a pytest-y manner?


def test_positive_zero():
    assert parse_number('0') == 0


@pytest.mark.parametrize(
    'in_string,out_number',
    [('1', 1),
     ('2', 2),
     ('5', 5),
     ('7', 7),
     ('9', 9)])
def test_positive_ones(in_string, out_number):
    assert parse_number(in_string) == out_number


@pytest.mark.parametrize(
    'in_string,out_number',
    [('10', 10),
     ('13', 13),
     ('15', 15),
     ('20', 20),
     ('29', 29),
     ('41', 41),
     ('50', 50),
     ('76', 76),
     ('95', 95),
     ('99', 99)])
def test_positive_tens(in_string, out_number):
    assert parse_number(in_string) == out_number


@pytest.mark.parametrize(
    'in_string,out_number',
    [('100', 100),
     ('277', 277),
     ('465', 465),
     ('511', 511),
     ('595', 595),
     ('791', 791),
     ('792', 792),
     ('850', 850),
     ('895', 895),
     ('999', 999)])
def test_positive_hundreds(in_string, out_number):
    assert parse_number(in_string) == out_number


@pytest.mark.parametrize(
    'in_string,out_number',
    [('1,000', 1000),
     ('1,067', 1067),
     ('32,712', 32712),
     ('104,001', 104001),
     ('613,533', 613533),
     ('999,999', 999999)])
def test_positive_thousands_full(in_string, out_number):
    assert parse_number(in_string) == out_number


@pytest.mark.parametrize(
    'in_string,out_number',
    [('1,000', 1000),
     ('1,067', 1067),
     ('32,712', 32712),
     ('104,001', 104001),
     ('613,533', 613533),
     ('999,999', 999999)])
def test_positive_thousands_abbr(in_string, out_number):
    assert parse_number(in_string) == out_number


@pytest.mark.parametrize(
    'in_string,out_number',
    [('1,000,000', 1000000),
     ('1,982,218', 1982218),
     ('12,345,678', 12345678),
     ('51,191,781', 51191781),
     ('64,374,237', 64374237),
     ('999,999,999', 999999999)])
def test_positive_millions_full(in_string, out_number):
    assert parse_number(in_string) == out_number


@pytest.mark.parametrize(
    'in_string,out_number',
    [('1m', 1000000),
     ('1.982m', 1982000),
     ('12.0m', 12000000),
     ('51.19m', 51190000),
     ('643.7m', 643700000),
     ('999.999999m', 999999999)])
def test_positive_millions_abbr(in_string, out_number):
    assert parse_number(in_string) == out_number


@pytest.mark.parametrize(
    'in_string,out_number',
    [('1,000,000,000', 1000000000)])
def test_positive_very_large_full(in_string, out_number):
    assert parse_number(in_string) == out_number


@pytest.mark.parametrize(
    'in_string,out_number',
    [('1.0b', 1000000000)])
def test_positive_very_large_abbr(in_string, out_number):
    assert parse_number(in_string) == out_number


def test_negative_zero():
    assert parse_number('-0') == 0


@pytest.mark.parametrize(
    'in_string,out_number',
    [('-1', -1),
     ('-2', -2),
     ('-5', -5),
     ('-7', -7),
     ('-9', -9)])
def test_negative_ones(in_string, out_number):
    assert parse_number(in_string) == out_number


@pytest.mark.parametrize(
    'in_string,out_number',
    [('-10', -10),
     ('-13', -13),
     ('-15', -15),
     ('-20', -20),
     ('-29', -29),
     ('-41', -41),
     ('-50', -50),
     ('-76', -76),
     ('-95', -95),
     ('-99', -99)])
def test_negative_tens(in_string, out_number):
    assert parse_number(in_string) == out_number


@pytest.mark.parametrize(
    'in_string,out_number',
    [('-100', -100),
     ('-277', -277),
     ('-465', -465),
     ('-511', -511),
     ('-595', -595),
     ('-791', -791),
     ('-792', -792),
     ('-850', -850),
     ('-895', -895),
     ('-999', -999)])
def test_negative_hundreds(in_string, out_number):
    assert parse_number(in_string) == out_number


@pytest.mark.parametrize(
    'in_string,out_number',
    [('-1,000', -1000),
     ('-1,067', -1067),
     ('-32,712', -32712),
     ('-104,001', -104001),
     ('-613,533', -613533),
     ('-999,999', -999999)])
def test_negative_thousands_full(in_string, out_number):
    assert parse_number(in_string) == out_number


@pytest.mark.parametrize(
    'in_string,out_number',
    [('-1k', -1000),
     ('-1.1k', -1100),
     ('-32.71k', -32710),
     ('-104.0k', -104000),
     ('-613.53k', -613530),
     ('-999.999k', -999999)])
def test_negative_thousands_abbr(in_string, out_number):
    assert parse_number(in_string) == out_number


@pytest.mark.parametrize(
    'in_string,out_number',
    [('-1,000,000', -1000000),
     ('-1,982,218', -1982218),
     ('-12,345,678', -12345678),
     ('-51,191,781', -51191781),
     ('-64,374,237', -64374237),
     ('-999,999,999', -999999999)])
def test_negative_millions_full(in_string, out_number):
    assert parse_number(in_string) == out_number


@pytest.mark.parametrize(
    'in_string,out_number',
    [('-1m', -1000000),
     ('-1.982m', -1982000),
     ('-12.0m', -12000000),
     ('-51.19m', -51190000),
     ('-643.7m', -643700000),
     ('-999.999999m', -999999999)])
def test_negative_millions_abbr(in_string, out_number):
    assert parse_number(in_string) == out_number


@pytest.mark.parametrize(
    'in_string,out_number',
    [('-1,000,000,000', -1000000000)])
def test_negative_very_large_full(in_string, out_number):
    assert parse_number(in_string) == out_number


@pytest.mark.parametrize(
    'in_string,out_number',
    [('-1.0b', -1000000000)])
def test_negative_very_large_abbr(in_string, out_number):
    assert parse_number(in_string) == out_number
