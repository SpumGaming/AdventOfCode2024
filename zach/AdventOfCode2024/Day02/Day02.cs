namespace AdventOfCode2024.Day02;

public class Day02 : Day 
{
	public Day02(string input) : base(input) 
	{
	}


	public bool IsLevelSafe(int[] level)
	{

		return true;
	}

	public bool IsJumpSafe(int[] nums, int i, int j, bool increasing)
	{
		int diff = nums[i] - nums[j];
		if ((diff > 0 && !increasing) || (diff < 0 && increasing)) 
		{
			return false;
		}

		int absDiff = Math.Abs(diff);
		if (!(absDiff >= 1 && absDiff <= 3)) 
		{
			return false;
		}

		return true;
	}

    public override string SolvePart1()
	{
		var lines = input.TrimEnd().Split(Environment.NewLine).ToArray();

		int safeReportsCount = 0;
		foreach (var line in lines)
		{
			int[] level = line.Split(" ").Select(x => int.Parse(x)).ToArray();
			int x = level[0];

			bool safe = true;

			bool increasing = (level[1] - level[0]) > 0;
			for (int i = 1; i < level.Length; i++) 
			{
				if (IsJumpSafe(level, i, i - 1, increasing))
				{
					continue;
				}
				else 
				{
					safe = false;
					break;
				}
			}

			if (safe)
			{
				safeReportsCount++;
			}
		}
		return safeReportsCount.ToString();
	}

    public override string SolvePart2()
	{
		var lines = input.TrimEnd().Split(Environment.NewLine).ToArray();

		int safeReportsCount = 0;
		foreach (var line in lines)
		{
			int[] level = line.Split(" ").Select(x => int.Parse(x)).ToArray();
			int x = level[0];

			bool safe = true;

			bool increasing = (level[1] - level[0]) > 0;
			for (int i = 1; i < level.Length; i++) 
			{
				if (IsJumpSafe(level, i, i - 1, increasing))
				{
					continue;
				}
				else 
				{
					safe = false;
					break;
				}
			}

			if (safe)
			{
				safeReportsCount++;
			}
		}
		return safeReportsCount.ToString();
	}
}
