package de.richardliebscher.rustjvm.py.collections;

import de.richardliebscher.rustjvm.py.PyBoolean;
import de.richardliebscher.rustjvm.py.PyException;
import de.richardliebscher.rustjvm.py.PyObject;

public interface PyContainer {
    PyBoolean __contains__(PyObject x) throws PyException;
}
