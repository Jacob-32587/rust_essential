namespace Program;
using System.Diagnostics;

public class Rectangle {
    double width;
    double height;

    public Rectangle(double width, double height) {
        this.width = width;
        this.height = height;
    }
    public double GetArea() {
        return width * height;
    }
    public void Scale(double scale) {
        width *= scale;
        height *= scale;
    } 
}

public class Program {
    public static void Main() {
        var rect = new Rectangle(1.2,3.4);
        Debug.Assert(rect.GetArea() == 4.08);
        rect.Scale(0.5);
        Debug.Assert(rect.GetArea() == 1.02);
        Console.WriteLine("Test passed!");
    }
}