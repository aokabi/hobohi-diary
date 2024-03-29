package app

import (
	"database/sql"
	"fmt"

	_ "github.com/go-sql-driver/mysql"
	_ "github.com/revel/modules"
	"github.com/revel/revel"
)

var (
	// AppVersion revel app version (ldflags)
	AppVersion string

	// BuildTime revel app build-time (ldflags)
	BuildTime string

	//Db
	DB *sql.DB

	//記事数
	EntryNum int
)

func InitDB() {
	address, _ := revel.Config.String("db.address")
	user, _ := revel.Config.String("db.username")
	pass, _ := revel.Config.String("db.password")
	database, _ := revel.Config.String("db.name")
	dbport, _ := revel.Config.String("db.port")
	revel.AppLog.Info(address)
	connstring := fmt.Sprintf("%s:%s@tcp(%s:%s)/%s?parseTime=true", user, pass, address, dbport, database)
	var err error
	DB, err = sql.Open("mysql", connstring)
	if err != nil {
		revel.AppLog.Info("DB Connect Error", err)
	}
	fmt.Printf("%s/n",DB)
	revel.AppLog.Info("DB Connected")
	row, err := DB.Query("SELECT COUNT(*) FROM entry")
	if err != nil {
		revel.AppLog.Info("DB Error", err)
	}
	defer row.Close()
	row.Next()
	row.Scan(&EntryNum)
}

func init() {
	// Filters is the default set of global filters.
	revel.Filters = []revel.Filter{
		revel.PanicFilter,             // Recover from panics and display an error page instead.
		revel.RouterFilter,            // Use the routing table to select the right Action
		revel.FilterConfiguringFilter, // A hook for adding or removing per-Action filters.
		revel.ParamsFilter,            // Parse parameters into Controller.Params.
		revel.SessionFilter,           // Restore and write the session cookie.
		revel.FlashFilter,             // Restore and write the flash cookie.
		revel.ValidationFilter,        // Restore kept validation errors and save new ones from cookie.
		revel.I18nFilter,              // Resolve the requested language
		HeaderFilter,                  // Add some security based headers
		revel.InterceptorFilter,       // Run interceptors around the action.
		revel.CompressFilter,          // Compress the result.
		revel.ActionInvoker,           // Invoke the action.
	}

	// add conf path
	revel.ConfPaths = []string{"conf", "conf/secret"}

	// Register startup functions with OnAppStart
	// revel.DevMode and revel.RunMode only work inside of OnAppStart. See Example Startup Script
	// ( order dependent )
	// revel.OnAppStart(ExampleStartupScript)
	revel.OnAppStart(InitDB)
	// revel.OnAppStart(FillCache)
}

// HeaderFilter adds common security headers
// There is a full implementation of a CSRF filter in
// https://github.com/revel/modules/tree/master/csrf
var HeaderFilter = func(c *revel.Controller, fc []revel.Filter) {
	c.Response.Out.Header().Add("X-Frame-Options", "SAMEORIGIN")
	c.Response.Out.Header().Add("X-XSS-Protection", "1; mode=block")
	c.Response.Out.Header().Add("X-Content-Type-Options", "nosniff")
	c.Response.Out.Header().Add("Referrer-Policy", "strict-origin-when-cross-origin")

	fc[0](c, fc[1:]) // Execute the next filter stage.
}

//func ExampleStartupScript() {
//	// revel.DevMod and revel.RunMode work here
//	// Use this script to check for dev mode and set dev/prod startup scripts here!
//	if revel.DevMode == true {
//		// Dev mode
//	}
//}
