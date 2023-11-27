{% for keyword in keywords -%}
#show regex("(?i)\\b{{keyword}}\\b"): set text(weight: "black", fill: rgb("#39cccc"))
{% endfor %}

#let page_title(body) = {
  set align(center)
  set text(
    size: 2.0em,
    weight: "medium",
  )
  [#body]
}

#let data = yaml("data.yaml")
#let theme = {
  let d = (:)
  let t = yaml("theme.yaml")
  for (key, value) in t {
    if key == "font" {
      d.insert(key, value)
    } else {
      d.insert(key, rgb(value))
    }
  }
  d
}

#show link: it => text(
  fill: theme.links,
  [#underline(it)]
)

#let website(item) = {
  link(item.url)[#item.network]
}

#let h1(body) = {
  set text(
    fill: theme.h1,
    size: 1.5em,
    weight: "medium",
  )
  [#box(width: 10pt, repeat[\_])#body#box(width: 1fr, repeat[\_])]
}

#let h2(body) = {
  set text(
    fill: theme.h2,
    size: 1.2em,
    weight: "semibold",
  )
  [#body]
}

#let cell = rect.with(
  width: 100%,
  stroke: none,
  radius: 2pt
)

#let rcell(body, inset: 2pt) = {
  set align(right)
  rect(
    inset: inset,
    width: 100%,
    stroke: none,
    radius: 2pt,
    [#body]
  )
}

#let indentedlist(items, indent: 5pt) = {
  set list(
    indent: indent,
  )
  for item in items {
    [- #item]
  }
}

#let summary(contents) = {
  h1("Summary")
  grid(
    columns: (3fr, 1fr),
    rows: (auto),
    column-gutter: 8pt,
    cell(
      inset: 0pt,
      stroke: (right: 0.5pt + theme.foreground),
      radius: 0pt,
    )[
      #grid(
        cell(
          stroke: none,
          inset: 5pt,
          par(
            justify: true,
            first-line-indent: 10pt,
            [ #contents.description ],
          )
        ),
        grid(
          row-gutter: 0pt,
          columns: (3fr, 4fr),
          rows: (auto),
          cell[#h2("Education")],
          cell[#align(right,
            contents.education,
          )],
          cell[#h2("Industry Experience")],
          cell[#align(right,
            contents.industry_experience,
          )],
        ),
      )
    ],
    cell(
      inset: (left: 0pt),
    )[
      #h2("Interests") \
      #v(10pt)
      #indentedlist(data.summary.interests)
    ]
  )
}

#let position(item) = {
  (
    cell(inset: 0pt)[
      #text(weight: "medium", item.title)
    ],
    rcell[
      #item.startDate - #item.endDate
    ],
  )
}

#let positions(items) = {
  grid(
    columns: (1fr, 1fr),
    rows: (auto),
    ..items.map(position).flatten(),
  )
}

#let work(item) = {
  grid(
    columns: 1,
    rows: (auto),
    column-gutter: 8pt,
    cell(inset: 0pt)[
      // company details
      #grid(
        columns: (1fr, 2fr),
        rows: (auto),
        cell(inset: 0pt)[
          #h2(item.name)
        ],
        cell(inset: 0pt)[
          #if item.keys().contains("url") {
            item.url
          }
        ],
      )
    ],
    cell(inset: 8pt)[
      #if item.at("positions", default: ()).len() > 0 {
        // positions
        positions(item.positions)
      } else {
        cell(inset: 0pt)[
          #text(weight: "medium", item.position)
        ]
      }
    ],
    if item.at("highlights", default: "").len() > 0 {
      cell(inset: 0pt)[
        // projects
        #indentedlist(item.highlights, indent: 13pt)
      ]
    },
  )
}

#let works(items) = {
  h1("Experience")
  for i in items {
    work(i)
  }
}

#let education(item) = {
  table(
    stroke: none,
    align: (ci, ri) => {
      if ci == 1 {
        left
      } else if ci == 2 and ri == 1 {
        center + horizon
      } else {
        right
      }
    },
    columns: (5pt, 8fr, 4fr),
    rows: (5pt, auto),
    [],
    [
      #h2(item.institution)
    ],
    h2(
    [
      #item.startDate - #item.endDate
    ]),
    [],
    rect(
      inset: 10pt,
      width: 100%,
      stroke: none,
      [
        #for degree in item.degrees {
          [
            #emph(degree) \
          ]
        }
      ],
    ),
    [
      GPA: #item.score
    ],
  )
}

#let educations(items) = {
  h1("Education")
  for i in items {
    education(i)
  }
}

#let project(item) = {
  rect(
    stroke: (left: 0.1pt + theme.foreground),
    [
      #align(left)[#link("https://"+item.url)[#item.url]]
      #if item.at("description", default: "").len() > 0 {
        par(
          first-line-indent: 5pt,
          justify: true,
            [#item.description],
        )
      }
    ],
  )
}

#let projects(items) = {
  h1("Open Source Projects")
  table(
    stroke: none,
    columns: (1fr, 1fr, 1fr),
    ..items.map(project),
  )
}

#let skills(items) = {
  h1("Skills")
  for i in items {
    skill(i)
  }
}

//
//
// Content
//
//

// global settings
#set text(
  fill: theme.foreground,
  font: (
  //theme.font,
  //"DejaVu Sans Mono",
  //"Luxi Mono",
  //"CodingFontTobi",
  //"C059",
  //"Andale Mono",
  //"Cantarell",
  //"ProggyCrossed",
  //"ProggyVector",
  "Verdana",
  "Liberation Mono",
  "FreeMono",
  ),
  size: 6pt,
)

#set page(
  paper: "us-letter",
  margin: 0.5in,
  fill: theme.background,
  header: [
    #set align(center)
    #data.basics.profiles.map(website).join(" - ")
    #v(5pt)
  ],
)

#page_title(data.basics.name)

#if data.at("summary", default: "").len() > 0 {
  summary(data.summary)
}

#if data.at("projects", default:"").len() > 0 {
  projects(data.projects)
}

#if data.work.len() > 0 {
  works(data.work)
}

#if data.education.len() > 0 {
  educations(data.education)
}

#if false and data.skills.len() > 0 {
  educations(data.skills)
}
