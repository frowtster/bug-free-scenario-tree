package main

import (
	"encoding/xml"
	"fmt"
	"os"
)

func main() {
	file, err := os.Open("scenario/first.xml")
	if err != nil {
		fmt.Printf("error: %v\n", err)
		return
	}
	defer file.Close()

	decoder := xml.NewDecoder(file)

	for {
		token, err := decoder.Token()
		if err != nil {
			break
		}

		switch se := token.(type) {
		case xml.StartElement:
			fmt.Printf("open: %s\n", se.Name.Local)
		case xml.EndElement:
			fmt.Printf("close: %s\n", se.Name.Local)
		}
	}
}
