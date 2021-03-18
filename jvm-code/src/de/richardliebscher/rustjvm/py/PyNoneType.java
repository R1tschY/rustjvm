package de.richardliebscher.rustjvm.py;

public class PyNoneType extends PyObject {
    public static final PyNoneType PY_NONE = new PyNoneType();
    static final PyType __CLASS__ = new PyType("NoneType");

    private PyNoneType() {

    }

    @Override
    public PyType __class__() {
        return __CLASS__;
    }

    @Override
    public PyString __repr__() {
        return new PyString("None");
    }
}
