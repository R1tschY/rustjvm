package de.richardliebscher.rustjvm.py;

public class IndexError extends RuntimeError {
    public IndexError(String message) {
        super(message);
    }
}
