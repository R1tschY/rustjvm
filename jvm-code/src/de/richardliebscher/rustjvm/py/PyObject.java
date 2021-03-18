package de.richardliebscher.rustjvm.py;

import java.lang.reflect.Method;
import java.util.ArrayList;
import java.util.List;
import java.util.stream.Collectors;

import static de.richardliebscher.rustjvm.py.NotImplementedType.NOT_IMPLEMENTED;
import static de.richardliebscher.rustjvm.py.PyNoneType.PY_NONE;

public class PyObject {
    private static final PyObject SENTINEL = new PyObject();

    private static PyType __CLASS__;

//    public PyObject __getattr__(String attr) throws PyException {
//        PyObject object = __dict__.getOrDefault(attr, SENTINEL);
//        if (object == SENTINEL) {
//            throw new AttributeError(String.format(
//                    "'%s' object has no attribute '%s'",
//                    __class__().getName(),
//                    attr));
//        } else {
//            return object;
//        }
//    }

    public PyNoneType __delattr__(PyObject obj) throws PyException {
        if (obj instanceof PyString) {
            __delattr__(((PyString) obj).asString());
            return PY_NONE;
        } else {
            throw new TypeError(String.format(
                    "attribute name must be string, not '%s'",
                    obj.__class__().getName()));
        }
    }

    public void __delattr__(String _attr) throws PyException {
        throw new TypeError(String.format(
                "can't set attributes of built-in/extension type '%s'",
                __class__().getName()));
    }

    public synchronized PyType __class__() {
        if (__CLASS__ == null) {
            __CLASS__ = new PyType("object");
        }

        return __CLASS__;
    }

    public PyInt __hash__() throws PyException {
        return PyInt.of(hashCode());
    }

    public PyObject __eq__(PyObject object) throws PyException {
        return NOT_IMPLEMENTED;
    }

    public PyString __str__() throws PyException {
        return __repr__();
    }

    public PyString __repr__() throws PyException {
        return new PyString(String.format(
                "<%s object at 0x%x>",
                __class__().getName(),
                System.identityHashCode(this)));
    }

    public PyObject __call__() throws PyException {
        return new PyObject();
    }

    public Method $getPyMethod(String attr, int pos_args) throws PyException {
        // search for builtin method
        Class<? extends PyObject> cls = getClass();

        Method[] methods = cls.getMethods();
        for (Method method : methods) {
            if (method.getName().equals(attr) && method.getParameterCount() == pos_args) {
                return method;
            }
        }

        List<Method> candidates = new ArrayList<>();
        for (Method method : methods) {
            if (method.getName().equals(attr)) {
                candidates.add(method);
            }
        }

        if (candidates.isEmpty()) {
            throw new AttributeError(String.format(
                    "'%s' object has no attribute '%s'",
                    __class__().getName(), attr));
        } else {
            throw new TypeError(String.format(
                    "'%s' object has no matching method, candidates are: %s",
                    __class__().getName(),
                    candidates.stream()
                            .map(Method::toString)
                            .collect(Collectors.joining("\n"))));
        }
    }

    @Override
    public String toString() {
        try {
            return __str__().asString();
        } catch (PyException e) {
            return String.format(
                    "<%s object at 0x%x>",
                    __class__().getName(),
                    System.identityHashCode(this));
        }
    }
}
