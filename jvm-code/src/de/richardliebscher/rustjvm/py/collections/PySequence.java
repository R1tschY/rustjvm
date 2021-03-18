package de.richardliebscher.rustjvm.py.collections;

import de.richardliebscher.rustjvm.py.PyException;
import de.richardliebscher.rustjvm.py.PyInt;
import de.richardliebscher.rustjvm.py.PyObject;
import de.richardliebscher.rustjvm.py.PySlice;

public interface PySequence extends PyCollection, PyReversible {
    PyObject __getitem__(PyInt i) throws PyException;
    PySequence __getitem__(PySlice i) throws PyException;

    PyInt index(PyObject value) throws PyException;
    PyInt index(PyObject value, PyInt start) throws PyException;
    PyInt index(PyObject value, PyInt start, PyInt end) throws PyException;
    PyObject count(PyObject value) throws PyException;


    static PySequence of(PyObject obj) throws PyException {
        if (obj instanceof PySequence) {
            return (PySequence) obj;
        } else {
            throw new RuntimeException();
        }
    }
}
