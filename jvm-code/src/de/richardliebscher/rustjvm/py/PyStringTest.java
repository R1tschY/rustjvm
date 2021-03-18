package de.richardliebscher.rustjvm.py;

import org.junit.jupiter.api.Test;

import java.util.ArrayList;
import java.util.List;

import static org.junit.jupiter.api.Assertions.*;
class PyStringTest {

    @Test
    void joinTwo() throws PyException {
        List<PyString> strings = new ArrayList<>();
        strings.add(PyString.of("abc"));
        strings.add(PyString.of("def"));

        assertEquals("abc, def", PyString.of(", ").join(
                JavaWrappers.toPyObject(strings)).asString());
    }

    @Test
    void joinEmpty() throws PyException {
        List<PyString> strings = new ArrayList<>();
        assertEquals("", PyString.of(", ").join(
                JavaWrappers.toPyObject(strings)).asString());
    }

    @Test
    void joinEmptySep() throws PyException {
        List<PyString> strings = new ArrayList<>();
        strings.add(PyString.of("abc"));
        strings.add(PyString.of("def"));

        assertEquals("abcdef", PyString.of("").join(
                JavaWrappers.toPyObject(strings)).asString());
    }

    @Test
    void joinJStrings() throws PyException {
        List<String> strings = new ArrayList<>();
        strings.add("abc");
        strings.add("def");

        assertEquals("abc/def", PyString.of("/").join(
                JavaWrappers.toPyObject(strings)).asString());
    }
}