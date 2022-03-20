class Solution:
    def findNumbers(self, nums: List[int]) -> int:
        evenDigits = 0

        for num in nums:
            if self.isEven(num):
                evenDigits += 1

        return evenDigits


    def isEven(self, num: int) -> bool:
        count = 0
        while num > 0:
            count += 1
            num %= 10
            print(num)
        return count%2==0


array = []
