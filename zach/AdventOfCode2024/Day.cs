namespace AdventOfCode2024;

public abstract class Day(string input)
{
   protected string input = input;
   public abstract string SolvePart1();
   public abstract string SolvePart2();


   protected char[,] GetInputAsCharGrid()
   {
      string[] lines = input.TrimEnd().Split(Environment.NewLine).ToArray();
      int width = lines[0].Length;
      int height = lines.Length;

      Console.WriteLine($"Width: {width}, Height: {height}");
      char[,] result = new char[width, height];
      for (int y = 0; y < height; y++)
      {
         for (int x = 0; x < width; x++)
         {
            result[x, y] = lines[y][x];
         }
      }
      return result;
   }

}
