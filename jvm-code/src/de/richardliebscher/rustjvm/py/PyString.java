package de.richardliebscher.rustjvm.py;

import de.richardliebscher.rustjvm.py.collections.PyCollection;
import de.richardliebscher.rustjvm.py.collections.PyIterable;
import de.richardliebscher.rustjvm.py.collections.PyIterator;

import java.util.ArrayList;
import java.util.List;

import static de.richardliebscher.rustjvm.py.PyBoolean.FALSE;

public class PyString extends PyObject {
    static final PyType __CLASS__ = new PyType("str");

    private final String inner;

    public PyString(String inner) {
        this.inner = inner;
    }

    @Override
    public PyType __class__() {
        return __CLASS__;
    }

    public static PyString of(String data) {
        return new PyString(data);
    }

    public String asString() {
        return inner;
    }

    @Override
    public PyString __repr__() throws PyException {
        // TODO: use escaping
        return PyString.of("'" + inner + "'");
    }

    @Override
    public PyInt __hash__() throws PyException {
        return PyInt.of(inner.hashCode());
    }

    @Override
    public PyObject __eq__(PyObject object) throws PyException {
        if (object instanceof PyString) {
            return PyBoolean.of(inner.equals(((PyString) object).inner));
        } else {
            return FALSE;
        }
    }

    public PyString join(PyObject obj) throws PyException {
        PyIterator iter = PyIterable.of(obj).__iter__();

        List<String> list;
        if (obj instanceof PyCollection) {
            list = new ArrayList<>(((PyCollection) obj).__len__().toInt());
        } else {
            list = new ArrayList<>();
        }

        for (int i = 0; true; ++i) {
            PyObject pyObject;
            try {
                pyObject = iter.__next__();
            } catch (StopIteration exp) {
                break;
            }

            if (pyObject instanceof PyString) {
                list.add(((PyString) pyObject).asString());
            } else {
                throw new TypeError(String.format(
                        "sequence item %d: expected str instance, %s found",
                        i, pyObject.__class__().getName()));
            }
        }
        return PyString.of(String.join(inner, list));
    }
}
