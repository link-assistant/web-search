> ## Documentation Index
> Fetch the complete documentation index at: https://docs.parallel.ai/llms.txt
> Use this file to discover all available pages before exploring further.

# Choose an API

> Find the right Parallel API for what you're building.

export const ApiChooser = () => {
  const jobs = {
    sources: {
      title: 'Find web sources for my agent',
      description: 'My agent will read the sources and write the answer.',
      question: 'Do you already have the URLs?',
      options: [{
        result: 'search',
        title: 'No, find relevant pages',
        description: 'Search for sources that answer my question.'
      }, {
        result: 'extract',
        title: 'Yes, read these URLs',
        description: 'Get the content from these pages.'
      }]
    },
    answer: {
      title: 'Get an answer with sources',
      description: 'Have Parallel do the research and write the answer.',
      question: 'When does the answer need to be ready?',
      options: [{
        result: 'responses',
        title: 'While my user or agent waits',
        description: 'Send the answer back in the same request.'
      }, {
        result: 'research',
        title: 'In the background',
        description: 'Let my app do other things while the research runs.'
      }]
    },
    enrich: {
      title: 'Fill in missing fields',
      description: 'Look up missing details for records I already have.',
      result: 'enrich'
    },
    discover: {
      title: 'Build a new list',
      description: 'Find people, companies, products, or papers that fit what I need.',
      question: 'Do you need a quick list or results checked against your criteria?',
      options: [{
        result: 'entities',
        title: 'A quick list of people or companies',
        description: "I'll check which results fit and look up any missing details."
      }, {
        result: 'findall',
        title: 'Check each result against my criteria',
        description: 'Have Parallel check the results for me. I can look for products, papers, and other things too.'
      }]
    },
    watch: {
      title: 'Track changes over time',
      description: "Let me know about new events or changes to something I've researched.",
      question: 'What should trigger an update?',
      options: [{
        result: 'events',
        title: 'A new event matches my query',
        description: 'Watch for news and announcements I care about.'
      }, {
        result: 'snapshot',
        title: 'An answer or field changes',
        description: 'Run my research again and tell me what changed.'
      }]
    }
  };
  const recommendations = {
    "search": {
      "name": "Search API",
      "why": "Search gives your agent sources to read. Your agent writes the answer.",
      "send": "Describe what you're researching and add focused search queries. Include enough context to make the subject clear.",
      "receive": "Ranked URLs, titles, and excerpts relevant to your question.",
      "own": "Your agent decides whether it has enough evidence, writes the answer, and cites the sources.",
      "configuration": "Set mode to fast and try a few of your own queries. If you leave mode out, it defaults to advanced.",
      "timing": "Results come back in the same request. The wait depends on the mode and whether pages need to be fetched live.",
      "verify": "Do the excerpts answer your question? Check that the sources support your claims. Use Extract if you need more of a page.",
      "billing": "You pay per request, based on the mode. Each request includes some results, with an extra charge if you go over that allowance.",
      "caution": "Search can use previously indexed content. If you need recent changes, set a fetch policy and check when it allows older content as a fallback.",
      "quickstart": "/search/search-quickstart",
      "setupHref": "/search/modes",
      "verifyHref": "/search/best-practices"
    },
    "extract": {
      "name": "Extract API",
      "why": "Use Extract to read pages when you already have the URLs.",
      "send": "One or more public URLs. Add an objective if you want excerpts focused on a particular question or section.",
      "receive": "Excerpts, plus page content if you ask for it. You'll also see which URLs couldn't be read.",
      "own": "Choose the pages to read. Your agent uses the content to write a summary or fill in the fields you need.",
      "configuration": "Start with excerpts. If you need more, set advanced_settings.full_content to true to get the page content Extract can read. Size limits still apply.",
      "timing": "Content comes back in the same request. The pages and fetch settings affect how long it takes. Some pages may fail to load.",
      "verify": "Make sure you got the section you need. Check each URL for errors and look for content cut off by size limits.",
      "billing": "You pay per URL, including URLs in a batch and pages you extract after a search.",
      "caution": "A URL alone won't give Extract access to private or paywalled content. It can also use previously indexed content, so check the freshness settings if you need recent changes.",
      "quickstart": "/extract/extract-quickstart",
      "setupHref": "/extract/advanced-extract-settings",
      "verifyHref": "/extract/best-practices"
    },
    "responses": {
      "name": "Responses API",
      "why": "Parallel researches your question and writes an answer while your user or agent waits.",
      "send": "Your question, any useful context, and a reasoning effort setting. Add an output schema if you need the answer in a specific format.",
      "receive": "An answer with citations, as text or structured output. Streaming lets your app show that the request has started. The full answer comes back when the research finishes.",
      "own": "Keep track of the conversation, show the citations, and decide whether to ask a follow-up question or have someone review the answer.",
      "configuration": "Try low for simple facts, medium for questions with several research steps, or high for deeper research. The default is medium.",
      "timing": "Expect roughly 5 to 60 seconds, depending on effort. Some requests take longer, so set your client's timeout above that.",
      "verify": "Open the cited sources and check that they support the answer's main claims before you use it.",
      "billing": "You pay per successful request, based on reasoning effort. There are no token charges. Each follow-up request costs extra.",
      "caution": "The request stays open until the answer is ready, even when you stream it. Use Task for research that should run in the background or in batches.",
      "quickstart": "/responses-api/responses-quickstart",
      "setupHref": "/responses-api/responses-quickstart",
      "verifyHref": "/responses-api/features/citations"
    },
    "research": {
      "name": "Task API",
      "why": "Task researches your question and writes a report in the background.",
      "send": "Your question, useful context, and a description or schema for the answer. Pick a processor based on how much research the question needs.",
      "receive": "A run ID first, then text or JSON when the research is done. The research basis gives you the citations, reasoning, and confidence ratings where available.",
      "own": "Save the run ID, handle failed runs, and fetch the answer when it's ready. Check the evidence before using the report.",
      "configuration": "Try pro for exploratory research. Check a few answers, then try ultra if you need more depth. The amount of research matters more than the number of fields.",
      "timing": "Research can take seconds to hours, plus time in the queue. Your app can do other things and fetch the answer when the run finishes.",
      "verify": "Read the sources and reasoning behind the report. Does it answer your question? Confidence ratings can help you choose what to review, but they don't prove an answer is right.",
      "billing": "You pay per successful run at the rate for your processor. Adding output fields doesn't add a charge, but starting another run does.",
      "caution": "Test a few questions before moving to a larger processor. Use Responses if your user or agent is waiting for the answer.",
      "quickstart": "/task-api/examples/task-deep-research",
      "setupHref": "/task-api/guides/choose-a-processor",
      "verifyHref": "/task-api/guides/access-research-basis"
    },
    "enrich": {
      "name": "Task API",
      "why": "Task looks up missing information for records you already have and returns the fields you asked for.",
      "send": "Enough detail to identify each record, such as a company name and website, plus an output schema. Explain what each field should contain and what evidence you need.",
      "receive": "A run ID, then the fields you requested with sources and reasoning to check before you update your records.",
      "own": "Match answers to the right records, handle batches and failed runs, and review fields that look wrong or incomplete.",
      "configuration": "Try base for standard enrichment, lite for simple lookups, or core for checking more sources. Test a few records first. The number of fields alone won't tell you which processor to use.",
      "timing": "The research runs in the background. The wait includes time in the queue and time spent researching. Get a notification when it's done, or check for results yourself.",
      "verify": "Check that you've got the right record and that the sources support each field. Use confidence ratings to guide your review, not as a pass/fail rule.",
      "billing": "You pay per successful run at the rate for your processor. One run can fill in several fields. Another run costs extra.",
      "caution": "If Parallel finds the wrong person or company, add details that make the record easier to identify before trying a larger processor.",
      "quickstart": "/task-api/examples/task-enrichment",
      "setupHref": "/task-api/guides/specify-a-task",
      "verifyHref": "/task-api/guides/access-research-basis"
    },
    "entities": {
      "name": "Entity Search",
      "why": "Use Entity Search for a quick list of people or companies. You decide which results fit.",
      "send": "Choose people or companies, describe who you're looking for, and set the maximum number of results.",
      "receive": "A ranked list of names, URLs, and descriptions. You may get fewer results than you asked for.",
      "own": "Keep the candidates that meet your requirements and look up any missing details you need.",
      "configuration": "Set entity_type, objective, and match_limit. Describe who you're looking for clearly. There's no processor or generator to choose.",
      "timing": "Usually takes seconds and returns one list in the same request. You can't page through more results.",
      "verify": "Check each result yourself. Being on the list doesn't mean it meets all your criteria. Use FindAll if you want Parallel to check them.",
      "billing": "You pay per request, with an extra charge for results beyond the included allowance. Task enrichment and FindAll runs cost extra.",
      "caution": "This API only finds people and companies. It doesn't include FindAll's checks, enrichment, or citations, so some results may not fit what you need.",
      "quickstart": "/findall-api/entity-search",
      "setupHref": "/findall-api/entity-search",
      "verifyHref": "/findall-api/entity-search"
    },
    "findall": {
      "name": "FindAll API",
      "why": "FindAll builds a list by searching for candidates and checking whether they meet your criteria.",
      "send": "Describe what you're looking for, spell out the match conditions, choose a generator, and set a match limit.",
      "receive": "A run ID, then candidates with match statuses and any available sources and reasoning. Filter for matched records to get the ones that passed your checks.",
      "own": "Make sure your criteria say what you mean. Collect the results, filter by match status, and check the evidence for the candidates you want to use.",
      "configuration": "Try preview to test your criteria on 5 to 10 candidates. Then choose a generator based on how hard it will be to find and verify matches.",
      "timing": "FindAll searches and checks candidates in the background. You can check its status, stream updates, or use webhooks.",
      "verify": "Read the match status and evidence for each candidate. A result snapshot can include unmatched candidates, so check the status before adding one to your list.",
      "billing": "You pay a fixed fee plus a fee per match, based on the generator. Each enrichment you add also has a Task processor charge for every match.",
      "caution": "Preview tests your criteria on candidates. It doesn't promise a set number of matches. A finished run doesn't mean you've found every match on the web, either.",
      "quickstart": "/findall-api/findall-quickstart",
      "setupHref": "/findall-api/core-concepts/findall-generator-pricing",
      "verifyHref": "/findall-api/core-concepts/findall-candidates"
    },
    "events": {
      "name": "Monitor API",
      "why": "Monitor checks for new events and tells your app when it finds something that matches your query.",
      "send": "Describe the events you care about, choose how often to check, and pick a monitor processor. Add a webhook URL for notifications.",
      "receive": "A monitor ID, then notifications when there are events to fetch. Events come in groups, with details and sources for each event.",
      "own": "Handle notifications and errors, fetch the events, and decide what to do with them. Cancel the monitor when you no longer need it.",
      "configuration": "Set type to event_stream. Try lite for a narrow query or base for a broader topic. Set the schedule based on how soon you need to know about an event.",
      "timing": "It checks once when you create it, then follows your schedule. Some checks won't find anything new.",
      "verify": "Check whether each run found events, found nothing new, or failed. Read the sources before acting on an alert.",
      "billing": "You pay for each check at the rate for your monitor processor, even if it finds no events. Checking more often costs more. Follow-up research and enrichment cost extra.",
      "caution": "Use this to watch for new events. It won't track every edit to a page or search for past events. If alerts stop, check whether the monitor is still running successfully.",
      "quickstart": "/monitor-api/monitor-quickstart",
      "setupHref": "/monitor-api/monitor-quickstart",
      "verifyHref": "/monitor-api/monitor-events"
    },
    "snapshot": {
      "name": "Monitor API",
      "why": "A snapshot monitor repeats your research and tells you when the answer changes, such as a price or who's running a company.",
      "send": "The ID of a completed Task run to use as the starting point, how often to check, and a monitor processor. That first run tells the monitor what to research again.",
      "receive": "A monitor ID, then what changed, the previous answer, and supporting sources and reasoning. Structured output makes it easier to see which fields changed.",
      "own": "Complete the first Task run, then create the monitor. Handle notifications and failures, fetch the changes, and check the evidence before updating your records.",
      "configuration": "Set type to snapshot and settings.task_run_id to the ID of your completed Task run. Start with a focused task and lite. Try base for broader checks.",
      "timing": "It repeats the research on your schedule and reports meaningful changes. The first Task run must finish before you create the monitor.",
      "verify": "Compare changed_output with previous_output and read the supporting evidence. Check the run status too. No alert could mean nothing changed, or it could mean a check failed.",
      "billing": "You pay for the first Task run and each scheduled Monitor check. The monitor processor sets the rate per check. Finding more changes doesn't add a charge.",
      "caution": "This compares research answers rather than page HTML. Use a structured schema to track individual fields. Text works too, but neither format is guaranteed to catch every change.",
      "quickstart": "/monitor-api/quickstart-snapshot",
      "setupHref": "/monitor-api/quickstart-snapshot",
      "verifyHref": "/monitor-api/monitor-events"
    }
  };
  const [selection, setSelection] = useState({
    job: null,
    result: null
  });
  const heading = useRef(null);
  const previousSelection = useRef(selection);
  const job = jobs[selection.job];
  const result = recommendations[selection.result];
  const track = (event, state, extra = {}) => {
    if (typeof window === 'undefined') return;
    window.ParallelConsent?.capture?.('api_chooser_' + event, {
      job: state.job,
      result: state.result,
      ...extra
    });
  };
  const choose = option => {
    const next = {
      job: option.job || selection.job,
      result: option.result || null
    };
    track('option_selected', next);
    setSelection(next);
  };
  useEffect(() => {
    if (previousSelection.current !== selection) {
      heading.current?.focus();
      previousSelection.current = selection;
    }
  }, [selection]);
  const back = () => setSelection({
    job: result && job.options ? selection.job : null,
    result: null
  });
  const reset = () => setSelection({
    job: null,
    result: null
  });
  const trackLink = (kind, href) => track('link_clicked', selection, {
    link_kind: kind,
    destination: href
  });
  const title = result ? 'Start with ' + result.name : job ? job.question : 'What are you building?';
  return <section className="api-chooser my-6 rounded-lg border border-gray-200 dark:border-[#2a2a2a] bg-white dark:bg-transparent p-5 sm:p-6" aria-label="API chooser">
      {job && <div className="mb-4 flex">
        {result && <button className="home-panel-link underline" type="button" onClick={back}><span aria-hidden="true">← </span>Back</button>}
        <button className="home-panel-link ml-auto underline" type="button" onClick={reset}>Start over</button>
      </div>}
      <h2 className="mt-0 mb-4 text-2xl tracking-tight text-gray-900 dark:text-gray-200 scroll-mt-28" ref={heading} tabIndex={-1}>{title}</h2>
      {result ? <div>
          <p className="block mb-4 text-base text-gray-600 dark:text-gray-400">{result.why}</p>
          <div className="mb-4 rounded-md bg-[#f6f6f6] dark:bg-[#1a1a1a] p-4">
            <h3 className="mt-0 mb-2 text-lg text-gray-900 dark:text-gray-200">How to start</h3>
            <p className="api-chooser-setup-description block mb-3 text-sm text-gray-600 dark:text-gray-400">{result.configuration}</p>
            <a className="home-panel-link" href={result.setupHref} onClick={() => trackLink('setup', result.setupHref)}>Setup details <span aria-hidden="true">→</span></a>
          </div>
          <a className="home-panel-link" href={result.quickstart} onClick={() => trackLink('quickstart', result.quickstart)}>Build with {result.name} <span aria-hidden="true">→</span></a>
          <dl>
            <div><dt>What you send</dt><dd>{result.send}</dd></div>
            <div><dt>What you get back</dt><dd>{result.receive}</dd></div>
            <div><dt>What your app handles</dt><dd>{result.own}</dd></div>
            <div><dt>How long it takes</dt><dd>{result.timing}</dd></div>
            <div><dt>What you pay for</dt><dd>{result.billing} <a href="/getting-started/pricing" onClick={() => trackLink('pricing', '/getting-started/pricing')}>Current pricing</a>.</dd></div>
            <div><dt>What to check</dt><dd>{result.verify} <a href={result.verifyHref} onClick={() => trackLink('verification', result.verifyHref)}>How to check results</a>.</dd></div>
          </dl>
          <p className="block text-sm text-gray-600 dark:text-gray-400"><strong>Before you build:</strong> {result.caution}</p>
        </div> : <div className="grid grid-cols-1 sm:grid-cols-2 gap-3">
          {(job ? job.options : Object.entries(jobs).map(([id, item]) => ({
    ...item,
    job: id
  }))).map(option => <button type="button" className="w-full text-left px-4 py-3 rounded transition-colors hover:bg-gray-50 dark:hover:bg-white/[0.03]" key={option.job || option.result} onClick={() => choose(option)}>
              <span className="block text-sm font-medium text-gray-900 dark:text-gray-100">{option.title}</span>
              <span className="block mt-0.5 text-sm leading-snug text-gray-500 dark:text-gray-400">{option.description}</span>
            </button>)}
        </div>}
    </section>;
};

<div className="sr-only" aria-hidden="false">
  For AI agents: a documentation index is available at [https://docs.parallel.ai/llms.txt](https://docs.parallel.ai/llms.txt). The full text of all docs is at [https://docs.parallel.ai/llms-full.txt](https://docs.parallel.ai/llms-full.txt). You may also fetch any page as Markdown by appending `.md` to its URL or sending `Accept: text/markdown`.

  For Search and Extract, use `/v1/search` and `/v1/extract` for all new integrations and agent tool calls. Use `/v1beta/search` and `/v1beta/extract` only when maintaining an existing integration that already uses them. The request bodies changed too: on `/v1/extract`, `full_content`, `fetch_policy` and `excerpt_settings` go inside `advanced_settings`, and sending the v1beta top-level `excerpts`, `full_content` and `fetch_policy` to `/v1/extract` returns a 422. Do not substitute `/v1` for the documented FindAll or Ingest endpoint paths.
</div>

<ApiChooser />

## Have your agent choose an API

Install the `choose-your-parallel-api` skill to help your coding agent choose the right
Parallel API and configuration for your use case. It considers Search and Extract, Responses,
and Task, then recommends a mode, reasoning effort, or processor based on your latency, cost,
and quality needs.

<Tabs>
  <Tab title="Claude Code">
    Install the skill globally so it is available in every project:

    ```bash theme={"system"}
    npx skills add parallel-web/parallel-agent-skills --skill choose-your-parallel-api --agent claude-code --global --yes
    ```
  </Tab>

  <Tab title="Codex">
    Install the skill globally so it is available in every project:

    ```bash theme={"system"}
    npx skills add parallel-web/parallel-agent-skills --skill choose-your-parallel-api --agent codex --global --yes
    ```
  </Tab>
</Tabs>

Remove `--global` to install the skill only in your current project. Once installed, describe
the integration you are building, including whether a user is waiting for the answer, whether
you already have URLs, and how many items you need to process.

<CardGroup cols={2}>
  <Card title="View the skill" icon="file-code" href="https://skills.parallel.ai/choose-your-parallel-api/SKILL.md">
    Read the complete, hosted `SKILL.md`.
  </Card>

  <Card title="Browse the source" icon="github" href="https://github.com/parallel-web/parallel-agent-skills/tree/main/skills/choose-your-parallel-api">
    Review the skill and its version history on GitHub.
  </Card>
</CardGroup>

## Go straight to a quickstart

Already know which API you want? Jump to its guide.

* [Search: find web sources](/search/search-quickstart)
* [Image Search: find images on the web](/image-search/image-search-quickstart)
* [Extract: get content from URLs](/extract/extract-quickstart)
* [Responses: get an answer with sources](/responses-api/responses-quickstart)
* [Task: research and enrich in the background](/task-api/task-quickstart)
* [Entity Search: find people and companies quickly](/findall-api/entity-search)
* [FindAll: build a list and check each result](/findall-api/findall-quickstart)
* [Monitor: watch for new events](/monitor-api/monitor-quickstart)

<Note>
  Choosing an option doesn't call any Parallel APIs. Try the suggested API with your own examples to see if the results, speed, and cost work for you. See [pricing](/getting-started/pricing) for current rates.
</Note>
