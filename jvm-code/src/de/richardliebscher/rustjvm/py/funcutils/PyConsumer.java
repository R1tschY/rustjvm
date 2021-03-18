package de.richardliebscher.rustjvm.py.funcutils;

import de.richardliebscher.rustjvm.py.PyException;
import de.richardliebscher.rustjvm.py.PyObject;

public interface PyConsumer {
    void accept(PyObject x) throws PyException;
}
