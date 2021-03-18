package de.richardliebscher.rustjvm.py.collections;

import de.richardliebscher.rustjvm.py.PyException;
import de.richardliebscher.rustjvm.py.PyObject;
import de.richardliebscher.rustjvm.py.StopIteration;
import de.richardliebscher.rustjvm.py.funcutils.PyConsumer;

import java.util.ArrayList;
import java.util.Collections;
import java.util.Iterator;
import java.util.List;
import java.util.function.Consumer;

public interface PyIterator extends PyIterable {
    PyObject __next__() throws PyException;

    default PyIterator __iter__() throws PyException {
        return this;
    }

    static PyIterator of(PyObject obj) throws PyException {
        if (obj instanceof PyIterator) {
            return (PyIterator) obj;
        } else {
            throw new RuntimeException();
        }
    }

    default void consume(PyConsumer consumer) throws PyException {
        PyObject obj;
        while (true) {
            try {
                obj = __next__();
            } catch (StopIteration exp) {
                return;
            }
            consumer.accept(obj);
        }
    }

    default void fastConsume(Consumer<PyObject> consumer) throws PyException {
        try {
            while (true) {
                consumer.accept(__next__());
            }
        } catch (StopIteration exp) {
            return;
        }
    }

    static List<PyObject> toJList(PyIterator iter) throws PyException {
        List<PyObject> list = new ArrayList<>();
        while (true) {
            try {
                list.add(iter.__next__());
            } catch (StopIteration exp) {
                return list;
            }
        }
    }

    static Iterator<PyObject> toJIterator(PyIterator iter) throws PyException {
        PyObject pyObject;
        try {
            pyObject = iter.__next__();
        } catch (StopIteration exp) {
            return Collections.emptyIterator();
        }

        return new Iterator<PyObject>() {
            private PyObject elem = pyObject;

            @Override
            public boolean hasNext() {
                return elem != null;
            }

            @Override
            public PyObject next() {
                if (elem != null) {
                    PyObject ret = elem;
                    try {
                        elem = iter.__next__();
                    } catch (StopIteration exp) {
                        elem = null;
                    } catch (PyException exp) {
                        throw new RuntimeException(exp);
                    }
                    return ret;
                } else {
                    throw new IllegalStateException("end of iterator");
                }
            }
        };
    }
}
