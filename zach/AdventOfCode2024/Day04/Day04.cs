namespace AdventOfCode2024.Day04;

public class Day04 : Day
{
	public Day04(string input) : base(input)
	{
	}


	public override string SolvePart1()
	{
		char[,] grid = GetInputAsCharGrid();

		int[,] dirs = new int[,] { { 0, 1 }, { 1, 0 }, { 0, -1 }, { -1, 0 }, { 1, 1 }, { -1, -1 }, { 1, -1 }, { -1, 1 } };

		string word = "XMAS";

		int count = 0;
		for (int i = 0; i < grid.GetLength(0); i++)
		{
			for (int j = 0; j < grid.GetLength(1); j++)
			{
				if (grid[i, j] != 'X')
				{
					continue;
				}

				for (int dir = 0; dir < 8; dir++)
				{
					bool foundWord = true;
					for (int k = 0; k < 4; k++)
					{
						int x = i + k * dirs[dir, 0];
						int y = j + k * dirs[dir, 1];

						if (x >= 0 && x < grid.GetLength(0) && y >= 0 && y < grid.GetLength(1) && grid[x, y] == word[k])
						{
							continue;
						}
						else
						{
							foundWord = false;
						}
					}

					if (foundWord)
					{
						count++;
					}
				}
			}
		}
		return count.ToString();
	}

	public override string SolvePart2()
	{
		char[,] grid = GetInputAsCharGrid();

		int[,] diagonal_dirs = new int[,] { { 1, 1 }, { -1, -1 }, { 1, -1 }, { -1, 1 } };

		int count = 0;
		for (int i = 0; i < grid.GetLength(0); i++)
		{
			for (int j = 0; j < grid.GetLength(1); j++)
			{
				if (grid[i, j] != 'A')
				{
					continue;
				}

				int m_count = 0;
				int s_count = 0;

				for (int dir = 0; dir < 4; dir++)
				{
					int x = i + diagonal_dirs[dir, 0];
					int y = j + diagonal_dirs[dir, 1];

					if (x >= 0 && x < grid.GetLength(0) && y >= 0 && y < grid.GetLength(1))
					{
						if (grid[x, y] == 'M')
						{
							m_count++;
						}
						else if (grid[x, y] == 'S')
						{
							s_count++;
						}
					}
					
				}

				if (m_count == 2 && s_count == 2)
				{
					if (!(grid[i + 1, j + 1] == grid[i - 1, j - 1]))
					{
						count++;
					}
				}
			}
		}

		return count.ToString();
	}
}
