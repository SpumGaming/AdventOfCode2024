using System.Text.RegularExpressions;

namespace AdventOfCode2024.Day03;

public class Day03 : Day 
{
	string pattern = @"(?:(mul\((\d+),(\d+)\))|(don't\(\))|(do\(\)))";
    private readonly Regex _regex;
	public Day03(string input) : base(input) 
	{
        _regex = new Regex(pattern, RegexOptions.Compiled);
	}

    public override string SolvePart1()
	{
		int result = 0;        
        foreach (Match match in _regex.Matches(input))
        {
            if (match.Groups[1].Success)
            {
                result += int.Parse(match.Groups[2].Value) * int.Parse(match.Groups[3].Value);
            }
        }

		return result.ToString();
	}
	
    public override string SolvePart2()
	{
		int result = 0;
        bool doing = true;
        
        foreach (Match match in _regex.Matches(input))
        {
            if (match.Groups[1].Success && doing)
            {
                result += int.Parse(match.Groups[2].Value) * int.Parse(match.Groups[3].Value);
            }

            if (match.Groups[4].Success)
            {
                doing = false;
            }
            
            if (match.Groups[5].Success)
            {
                doing = true;
            }
        }
		return result.ToString();
	}
}
