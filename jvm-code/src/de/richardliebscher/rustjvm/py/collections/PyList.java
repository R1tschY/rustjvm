package de.richardliebscher.rustjvm.py.collections;

import de.richardliebscher.rustjvm.py.*;

import java.util.ArrayList;
import java.util.Collections;
import java.util.Iterator;
import java.util.List;

import static de.richardliebscher.rustjvm.py.PyNoneType.PY_NONE;

public class PyList extends PyObject implements PySequence {
    static final PyType __CLASS__ = new PyType("list");
    @Override
    public PyType __class__() {
        return __CLASS__;
    }

    private final List<PyObject> inner;

    public PyList() {
        this.inner = new ArrayList<>();
    }

    private PyList(List<PyObject> inner) {
        this.inner = inner;
    }

    public void append(PyObject obj) {
        inner.add(obj);
    }

    public void extend(PyObject obj) throws PyException {
        // TODO: use addAll
        PyIterable.of(obj).fastConsume(inner::add);
    }

    public void clear() {
        inner.clear();
    }

    public void reverse() {
        Collections.reverse(inner);
    }

    public PyList copy() {
        return new PyList(new ArrayList<>(inner));
    }

    @Override
    public PyObject __getitem__(PyInt i) throws PyException {
        return null;
    }

    @Override
    public PySequence __getitem__(PySlice i) throws PyException {
        return null;
    }

    @Override
    public PyInt index(PyObject value) throws PyException {
        return null;
    }

    @Override
    public PyInt index(PyObject value, PyInt start) throws PyException {
        return null;
    }

    @Override
    public PyInt index(PyObject value, PyInt start, PyInt end) throws PyException {
        return null;
    }

    @Override
    public PyObject count(PyObject value) throws PyException {
        return null;
    }

    @Override
    public PyInt __len__() {
        return PyInt.of(inner.size());
    }

    @Override
    public PyBoolean __contains__(PyObject x) throws PyException {
        return null;
    }

    @Override
    public PyIterator __reversed__() throws PyException {
        return null;
    }

    @Override
    public PyIterator __iter__() {
        return new ListIterator(inner.iterator());
    }

    @Override
    public PyString __repr__() throws PyException {
        List<String> elems = new ArrayList<>(inner.size());
        for (PyObject o : inner) {
            elems.add(o.__repr__().asString());
        }
        return PyString.of("[" + String.join(", ", elems) + "]");
    }

    static final class ListIterator extends PyObject implements PyIterator {
        static final PyType __CLASS__ = new PyType("list_iterator");
        @Override
        public PyType __class__() {
            return __CLASS__;
        }

        private final Iterator<PyObject> inner;

        public ListIterator(Iterator<PyObject> inner) {
            this.inner = inner;
        }

        @Override
        public PyObject __next__() throws PyException {
            if (inner.hasNext()) {
                return inner.next();
            } else {
                throw new StopIteration();
            }
        }
    }
}
