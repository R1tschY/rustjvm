package de.richardliebscher.rustjvm.py;

import de.richardliebscher.rustjvm.py.collections.PyIterable;
import de.richardliebscher.rustjvm.py.collections.PySequence;
import org.junit.jupiter.api.Test;

import java.util.ArrayList;
import java.util.List;

import static de.richardliebscher.rustjvm.py.JavaWrappers.toPyObject;
import static org.junit.jupiter.api.Assertions.*;

class JavaWrappersTest {

    @Test
    void checkListWrapper() throws PyException {
        List<Integer> list = new ArrayList<>();
        list.add(1);
        list.add(2);
        list.add(3);

        PySequence pyObject = PySequence.of(toPyObject(list));
        assertEquals(3, pyObject.__len__().toInt());
    }
}