package de.richardliebscher.rustjvm.py.collections;

import de.richardliebscher.rustjvm.py.PyException;
import de.richardliebscher.rustjvm.py.PyInt;
import org.junit.jupiter.api.Test;

import static org.junit.jupiter.api.Assertions.*;

class PyListTest {

    @Test
    void checkAppend() {
        PyList list = new PyList();
        list.append(PyInt.of(1));
        list.append(PyInt.of(1));
        list.append(PyInt.of(1));

        assertEquals(3, list.__len__().toInt());
    }

    @Test
    void checkExtend() throws PyException {
        PyList list = new PyList();
        list.append(PyInt.of(1));
        list.append(PyInt.of(1));
        list.append(PyInt.of(1));

        PyList list2 = new PyList();
        list2.append(PyInt.of(3));
        list2.extend(list);

        assertEquals(4, list2.__len__().toInt());
    }
}