package de.richardliebscher.rustjvm.py;

import java.lang.reflect.InvocationTargetException;
import java.lang.reflect.Method;

public final class PyReflectionHelper {

    private PyReflectionHelper() {

    }

    PyObject call_method(Method method, PyObject self, PyObject... args) throws PyException {
        Object ret;
        try {
            ret = method.invoke(self, args);
        } catch (IllegalAccessException e) {
            throw new RuntimeException(e);
        } catch (InvocationTargetException e) {
            throw new TypeError(e.getMessage());
        }

        if (ret instanceof PyObject) {
            return (PyObject) ret;
        } else {
            PyObject pyObject = JavaWrappers.toPyObject(ret);
            if (pyObject == null) {
                throw new RuntimeException(String.format(
                        "illegal return for python method call: %s",
                        ret.toString()));
            }
            return pyObject;
        }
    }
}
