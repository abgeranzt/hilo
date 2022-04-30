#!/bin/python3
import re

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
		self.size = size
		for c in ["a", "b", "c", "d"]:
			val = 14
			for i in range(int(size / 4)):
				self.cards[f'{c}{val}'] = True
				val -= 1

	def print(self):
		for i in range(15 - int(self.size / 4), 15):
			cards = []
			for c in ["a", "b", "c", "d"]:
				card = f"{c}{i}"
				cards.append(self.cards[card] and card or "")
			print("\t".join(cards))

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

def game_init():
	deck = None
	while True:
		size = 0
		try:
			size = int(input("Deck size: "))
			if size > 52 or size % 4 != 0:
				raise TypeError
		except TypeError:
			print("Invalid input. (ex: 24, 32, 36, 52)")
			continue
		deck = Deck(size)
		break
	rows = None
	while True:
		try:
			rows = int(input("Rows: "))
			if rows > deck.size:
				raise TypeError
		except TypeError:
			print("Invalid input. (ex: 4, 5, 6)")
			continue
		break
	table = None
	while True:
		cards = None
		try:
			cards = re.split(",", input("Initial cards: "))
			for c in cards:
				print(c)
				if not re.match('^[abc]\d{1,2}$', c):
					raise TypeError
		except TypeError:
			print("Invalid input. (ex: a11,c6,d8)")
			continue
		for c in cards:
			deck.remove(c)
		table = Table(rows, cards)
		break
	return deck, table

def game_loop():
	pass

def main():
	deck, table = game_init()
	table.print()
#	deck = Deck(52)
	deck.print()

if __name__ == '__main__':
	main()
