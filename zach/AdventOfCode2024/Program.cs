using System.Collections.Frozen;
using System.Reflection;
using System.Text.RegularExpressions;

namespace AdventOfCode2024;

public static partial class Program
{
    [GeneratedRegex(@"^Day\d{2}$")]
    private static partial Regex DayPattern();

    static void Main(string[] args)
    {
        Type[] days = Assembly
            .GetExecutingAssembly().GetTypes()
            .Where(x => DayPattern().IsMatch(x.Name))
            .OrderBy(x => int.Parse(x.Name[3..]))
            .ToArray();

        if (args.Length < 1)
        {
            FrozenSet<int> doneDays = days.Select(x => int.Parse(x.Name[3..])).Order().ToFrozenSet();

            Console.WriteLine("Please specify a day to run.");
            Console.WriteLine("Possible days:");
            for (int i = 1; i <= 25; i++)
            {
                Console.Write(doneDays.Contains(i) ? $"{i,2} " : "   ");
                if (i % 5 == 0)
                {
                    Console.WriteLine();
                }
            }
            // Console.WriteLine("{0}", string.Join(", ", problemList.Select(x => int.Parse(x.Name.Substring(3))).Order()));
            return;
        }

        bool test = args.Length > 1 && (args[1] == "test" || args[1] == "t" || args[1] == "true");
        int day;

        if (args[0].Equals("all"))
        {
            foreach (Type type in days)
            {
                day = int.Parse(type.Name.Substring(3));
                TryRunProblem(day, days, test);
            }
            return;
        }

        day = int.Parse(args[0]);

        string? customInput = args.Length >= 2 && !test ? args[1] : null;
        TryRunProblem(day, days, test, customInput);
    }

    private static void TryRunProblem(int day_number, Type[] problemList, bool test, string? additionalInput = null)
    {
        string inputPath = test ? $"Day{day_number:D2}/test.txt" : $"Day{day_number:D2}/input.txt";

        if (!File.Exists(inputPath))
        {
            Console.WriteLine($"Could not read \'{inputPath}\'.");
            return;
        }

        Type type = problemList.First(x => x.Name == $"Day{day_number:D2}");
        string input = additionalInput == null ? File.ReadAllText(inputPath) : additionalInput;

        if (Activator.CreateInstance(type, input) is Day day)
        {
            PrintProblemOutput(day);
        }
        else
        {
            Console.WriteLine($"Instance of {type.Name} could not be created.");
            return;
        }
    }

    private static void PrintProblemOutput(Day day)
    {
        // TODO: Add timing
        Console.WriteLine(day.GetType().Name);
        string part1 = day.SolvePart1();
        string part2 = day.SolvePart2();

        Console.WriteLine($"Part 1: {part1}");
        Console.WriteLine($"Part 2: {part2}");
    }
}
