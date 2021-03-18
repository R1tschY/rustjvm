package de.richardliebscher.rustjvm.py.collections;

import de.richardliebscher.rustjvm.py.PyException;
import de.richardliebscher.rustjvm.py.PyObject;
import de.richardliebscher.rustjvm.py.funcutils.PyConsumer;

import java.util.function.Consumer;

public interface PyIterable {
    PyIterator __iter__() throws PyException;

    static PyIterable of(PyObject obj) throws PyException {
        if (obj instanceof PyIterable) {
            return (PyIterable) obj;
        } else {
            throw new RuntimeException();
        }
    }

    default void consume(PyConsumer consumer) throws PyException {
        __iter__().consume(consumer);
    }

    default void fastConsume(Consumer<PyObject> consumer) throws PyException {
        __iter__().fastConsume(consumer);
    }
}
