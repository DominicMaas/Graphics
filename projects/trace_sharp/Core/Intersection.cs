namespace TraceSharp.Core;

public class Intersection
{
    public Intersection(float distance, RenderObject obj)
    {
        Distance = distance;
        Object = obj;
    }

    public float Distance { get; }
    public RenderObject Object { get; }
}
