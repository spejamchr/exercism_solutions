using System;
using System.Collections;
using System.Collections.Generic;

public class SimpleLinkedList<T> : IEnumerable<T>
{
    private class Node
    {
        public T value;
        public Node Next;

        public Node(T value) => this.value = value;
    }

    public int Count { get; private set; } = 0;
    private Node Head;

    public SimpleLinkedList() { }

    public SimpleLinkedList(T item) => Push(item);

    public SimpleLinkedList(IEnumerable<T> items)
    {
        foreach (var item in items)
            Push(item);
    }

    public void Push(T value)
    {
        Count += 1;
        var newHead = new Node(value);
        newHead.Next = Head;
        Head = newHead;
    }

    public T Pop()
    {
        if (Count == 0)
            throw new InvalidOperationException("Cannot call Pop() on an empty list.");

        Count -= 1;
        var oldHead = Head;
        Head = oldHead.Next;
        return oldHead.value;
    }

    public IEnumerator<T> GetEnumerator()
    {
        var node = Head;
        while (node != null)
        {
            yield return node.value;
            node = node.Next;
        }
    }

    IEnumerator IEnumerable.GetEnumerator() => this.GetEnumerator();
}
