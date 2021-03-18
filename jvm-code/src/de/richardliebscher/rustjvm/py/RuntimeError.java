package de.richardliebscher.rustjvm.py;

public class RuntimeError extends PyException {
    private final String message;

    public RuntimeError(String message) {
        this.message = message;
    }
}
