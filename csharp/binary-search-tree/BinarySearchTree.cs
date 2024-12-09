using System.Collections;
using System.Collections.Generic;
using System.Linq;

public class BinarySearchTree : IEnumerable<int>
{
    public int Value { get; private set; }
    public BinarySearchTree Left { get; private set; }
    public BinarySearchTree Right { get; private set; }

    public BinarySearchTree(int value) => Value = value;

    public BinarySearchTree(IEnumerable<int> values)
    {
        Value = values.First();
        foreach (var item in values.Skip(1))
            Add(item);
    }

    public BinarySearchTree Add(int value)
    {
        if (value <= Value && Left == null)
            Left = new BinarySearchTree(value);
        else if (value <= Value)
            Left.Add(value);
        else if (Right == null)
            Right = new BinarySearchTree(value);
        else
            Right.Add(value);

        return this;
    }

    public IEnumerator<int> GetEnumerator()
    {
        if (Left != null)
            foreach (var item in Left)
                yield return item;

        yield return Value;

        if (Right != null)
            foreach (var item in Right)
                yield return item;
    }

    IEnumerator IEnumerable.GetEnumerator() => this.GetEnumerator();
}
