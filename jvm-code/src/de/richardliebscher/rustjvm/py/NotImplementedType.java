package de.richardliebscher.rustjvm.py;

import java.math.BigInteger;

public class NotImplementedType extends PyObject {
    public static final NotImplementedType NOT_IMPLEMENTED = new NotImplementedType();

    static final PyType __CLASS__ = new PyType("NotImplementedType");

    @Override
    public PyType __class__() {
        return __CLASS__;
    }
}
