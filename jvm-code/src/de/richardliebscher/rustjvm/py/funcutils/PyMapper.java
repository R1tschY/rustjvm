package de.richardliebscher.rustjvm.py.funcutils;

import de.richardliebscher.rustjvm.py.PyException;
import de.richardliebscher.rustjvm.py.PyObject;

public interface PyMapper {
    PyObject apply(PyObject x) throws PyException;
}
