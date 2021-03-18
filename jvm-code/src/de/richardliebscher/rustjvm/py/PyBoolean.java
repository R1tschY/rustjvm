package de.richardliebscher.rustjvm.py;

import java.math.BigInteger;

public class PyBoolean extends PyObject {
    static final PyBoolean TRUE = new PyBoolean(true);
    static final PyBoolean FALSE = new PyBoolean(false);
    static final PyType __CLASS__ = new PyType("bool");

    private final boolean inner;

    private PyBoolean(boolean inner) {
        this.inner = inner;
    }

    public static PyBoolean of(boolean value) {
        return value ? TRUE : FALSE;
    }

    public boolean toBoolean() {
        return inner;
    }

    @Override
    public PyType __class__() {
        return __CLASS__;
    }
}
