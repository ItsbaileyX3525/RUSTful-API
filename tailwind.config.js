module.exports = {
	content: [
		"./templates/**/*.{html,tera,hbs}", // your template files
		"./src/**/*.rs", // if you embed classes in Rust code
	],
	theme: {
		extend: {},
	},
	plugins: [],
}
