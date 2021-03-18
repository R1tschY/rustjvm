package de.richardliebscher.rustjvm.py.collections;

import de.richardliebscher.rustjvm.py.PyException;
import de.richardliebscher.rustjvm.py.PyInt;

public interface PyCollection extends PyIterable, PyContainer {
    PyInt __len__() throws PyException;
}
