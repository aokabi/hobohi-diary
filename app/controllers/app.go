package controllers

import (
	"strings"
	"time"

	"github.com/revel/revel"
)

type App struct {
	*revel.Controller
}

type Entry struct {
	Day     time.Time
	Content []string
}

var (
	e []Entry
)

func (c App) Index() revel.Result {
	return c.Render()
}

func (c App) Post() revel.Result {
	s := strings.Split(c.Params.Form.Get("content"), "\n")
	d := time.Now()
	if len(e) == 0 {
		e = append(e, Entry{d, s})
	} else {
		e = append([]Entry{Entry{d, s}}, e...)
	}
	c.ViewArgs["entry"] = e
	return c.RenderTemplate("App/Index.html")
}
