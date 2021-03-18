package de.richardliebscher.rustjvm.py;

import de.richardliebscher.rustjvm.py.collections.PyIterator;
import de.richardliebscher.rustjvm.py.collections.PySequence;

import java.util.HashMap;
import java.util.Iterator;
import java.util.List;
import java.util.function.Function;

import static de.richardliebscher.rustjvm.py.PyNoneType.PY_NONE;

public class JavaWrappers {
    static HashMap<Class<?>, Function<Object, PyObject>> PY = new HashMap<>();

    private JavaWrappers() {

    }

    static  {
        PY.put(Byte.class, obj -> PyInt.of((Byte) obj));
        PY.put(Short.class, obj -> PyInt.of((Short) obj));
        PY.put(Integer.class, obj -> PyInt.of((Integer) obj));
        PY.put(Long.class, obj -> PyInt.of((Long) obj));
        PY.put(Boolean.class, obj -> PyBoolean.of((Boolean) obj));
        PY.put(String.class, obj -> PyString.of((String) obj));
    }

    static PyObject toPyObject(Object obj) {
        if (obj instanceof PyObject) {
            return (PyObject) obj;
        } else if (obj == null) {
            return PY_NONE;
        } else {
            Function<Object, PyObject> converter = PY.get(obj.getClass());
            if (converter != null) {
                return converter.apply(obj);
            } else if (obj instanceof List) {
                return new ListWrapper((List<?>) obj);
            } else if (obj instanceof Iterator) {
                return new IteratorWrapper((Iterator<?>) obj);
            }
        }

        throw new RuntimeException(String.format(
                "'%s' object can not be used as python object",
                obj.getClass().getName()));
    }

    static final class ListWrapper extends PyObject implements PySequence {
        private final List<?> inner;

        public ListWrapper(List<?> obj) {
            inner = obj;
        }

        @Override
        public PyObject __getitem__(PyInt i) throws PyException {
            try {
                return toPyObject(inner.get(i.toInt()));
            } catch (IndexOutOfBoundsException | ArithmeticException exp) {
                throw new IndexError("list index out of range");
            }
        }

        @Override
        public PySequence __getitem__(PySlice i) throws PyException {
            return null;
        }

        @Override
        public PyInt index(PyObject value) throws PyException {
            return null;
        }

        @Override
        public PyInt index(PyObject value, PyInt start) throws PyException {
            return null;
        }

        @Override
        public PyInt index(PyObject value, PyInt start, PyInt end) throws PyException {
            return null;
        }

        @Override
        public PyObject count(PyObject value) throws PyException {
            return null;
        }

        @Override
        public PyInt __len__() throws PyException {
            return new PyInt(inner.size());
        }

        @Override
        public PyIterator __iter__() throws PyException {
            return new IteratorWrapper(inner.iterator());
        }

        @Override
        public PyIterator __reversed__() throws PyException {
            return null;
        }

        @Override
        public PyBoolean __contains__(PyObject x) throws PyException {
            return null;
        }
    }

    static final class IteratorWrapper  extends PyObject implements PyIterator {
        private final Iterator<?> inner;

        public IteratorWrapper(Iterator<?> inner) {
            this.inner = inner;
        }

        @Override
        public PyObject __next__() throws PyException {
            if (inner.hasNext()) {
                return toPyObject(inner.next());
            } else {
                throw new StopIteration();
            }
        }
    }
}
