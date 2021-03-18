package de.richardliebscher.rustjvm.py.collections;

import de.richardliebscher.rustjvm.py.PyException;

public interface PyReversible extends PyIterable {
    PyIterator __reversed__() throws PyException;
}
