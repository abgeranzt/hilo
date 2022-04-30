#!/bin/python3

"""
a = Clubs
b = Spades
c = Hearts
d = Diamonds
"""

class Deck:
	cards = {}
	size = {}

	def __init__(self, size):
		if size % 4 > 0 or size > 52:
			raise("Invalid deck size")
		self.size = size
		for c in ["a", "b", "c", "d"]:
			val = 14
			for i in range(int(size / 4)):
				self.cards[f'{c}{val}'] = True
				val -= 1

	def add(self, card):
		self.cards[card] = True
		self.size += 1

	def remove(self, card):
		self.cards[card] = False
		self.size -= 1

	def calc(self, card):
		val = int(card[1:])
		higher, lower, equal = 0, 0, 0
		for c in self.cards:
			if not self.cards[c]:
				continue
			if val < int(c[1:]):
				higher += 1
			elif val == int(c[1:]):
				equal += 1
			else:
				lower += 1
		chance = lambda n : n / self.size
		return chance(higher), chance(lower), chance(equal)

class Table:
	rows = []

	def __init__(self, rows, cards):
		for i in range(rows):
			self.rows.append([cards[i]])

	def print(self):
		for row in self.rows:
			print(row)

	def add_left(self, row, card):
		self.rows[row].insert(0, card)

	def add_right(self, row, card):
		self.rows[row].append(card)

def main():
	deck = Deck(8)
	deck.remove('a14')
	print(deck.calc('a14'))

if __name__ == '__main__':
	main()
