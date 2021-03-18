package de.richardliebscher.rustjvm.py;

import java.math.BigInteger;

public class PyInt extends PyObject {
    static final PyType __CLASS__ = new PyType("int");

    private final BigInteger inner;

    public PyInt(BigInteger inner) {
        this.inner = inner;
    }

    public PyInt(long inner) {
        this.inner = BigInteger.valueOf(inner);
    }

    public static PyInt of(long value) {
        return new PyInt(value);
    }

    public int toInt() {
        return inner.intValueExact();
    }

    @Override
    public PyType __class__() {
        return __CLASS__;
    }
}
