package main

type Cache struct {
	storage map[string]Response
}

func NewCache() *Cache {
	return &Cache{
		storage: make(map[string]Response),
	}
}

type Response struct {
	StatusCode int
	Headers    map[string]string
	Body       []byte
}

func (c *Cache) Set(key string, value Response) {
	c.storage[key] = value
}

func (c *Cache) Get(key string) (Response, bool) {
	value, ok := c.storage[key]
	return value, ok
}

func (c *Cache) Delete(key string) {
	delete(c.storage, key)
}

func (c *Cache) Clear() {
	c.storage = make(map[string]Response)
}
