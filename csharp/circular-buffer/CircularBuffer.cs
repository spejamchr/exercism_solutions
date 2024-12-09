using System;
using System.Collections.Generic;

public class CircularBuffer<T>
{
    private List<T> container;
    private int oldestIdx = 0;
    private int nextIdx = 0;
    public int Count = 0;

    public CircularBuffer(int capacity)
    {
        container = new List<T>(capacity);
    }

    public T Read()
    {
        if (Count == 0)
            throw new InvalidOperationException("Cannot read from an empty CircularBuffer");

        Count -= 1;
        oldestIdx = increment(oldestIdx);
        return container[decrement(oldestIdx)];
    }

    public void Write(T value)
    {
        if (container.Count < container.Capacity)
        {
            container.Add(value);
        }
        else if (Count < container.Capacity)
        {
            container[nextIdx] = value;
        }
        else
        {
            throw new InvalidOperationException("Cannot write to a full CircularBuffer");
        }

        nextIdx = increment(nextIdx);
        Count += 1;
    }

    public void Overwrite(T value)
    {
        if (Count == container.Capacity)
            Read();

        Write(value);
    }

    public void Clear()
    {
        Count = 0;
        oldestIdx = nextIdx;
    }

    private int increment(int n) => (n + 1) % container.Capacity;

    private int decrement(int n) => (n + container.Capacity - 1) % container.Capacity;
}
