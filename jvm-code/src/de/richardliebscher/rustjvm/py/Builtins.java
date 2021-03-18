package de.richardliebscher.rustjvm.py;

import de.richardliebscher.rustjvm.py.collections.PyCollection;

public class Builtins {

    public static PyString str(PyObject obj) throws PyException {
        return obj.__str__();
    }

    public static PyString repr(PyObject obj) throws PyException {
        return obj.__repr__();
    }

    public static final PyType type = PyType.INSTANCE;
    public static final PyType object = new PyObject().__class__();

    public static PyInt len(PyObject obj) throws PyException {
        if (obj instanceof PyCollection) {
            return ((PyCollection) obj).__len__();
        } else {
            throw new TypeError(String.format(
                    "object of type '%s' has no len()",
                    obj.__class__().getName()));
        }
    }
}
