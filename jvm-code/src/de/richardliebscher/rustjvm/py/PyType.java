package de.richardliebscher.rustjvm.py;

public class PyType extends PyObject {
    static final PyType INSTANCE = new PyType("type");
    private static final PyType __CLASS__ = INSTANCE;

    private final String name;

    public PyType(String name) {
        this.name = name;
    }

    public String getName() {
        return name;
    }

    public PyString __name__() {
        return PyString.of(name);
    }

    @Override
    public PyType __class__() {
        return __CLASS__;
    }
}
